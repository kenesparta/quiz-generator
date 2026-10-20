use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::respuesta::domain::entity::revision::{ExamenRevision, Firma, RevisionRealizada};
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::{RepositorioFirma, RepositorioRealizarRevision};
use chrono::{DateTime, FixedOffset};

pub struct InputData {
    pub respuesta_id: String,
    pub evaluacion_id: String,
    pub resultado: String,
    pub examenes: Vec<InputDataExamen>,
    /// El psicólogo que califica: el sujeto del token. Solo un psicólogo califica, porque la
    /// revisión lleva su firma.
    pub psicologo_id: PsicologoID,
    pub ahora: DateTime<FixedOffset>,
}

pub struct InputDataExamen {
    pub examen_id: String,
    pub observacion: String,
}

pub struct OutputData {
    /// La firma que quedó guardada en la hoja.
    pub firma: Firma,
}

/// Califica una hoja finalizada. Solo se califica lo que el postulante ya no puede cambiar.
///
/// La revisión lleva la firma del psicólogo que califica (su nombre y su colegiatura en ese
/// momento) y se guarda con ella.
///
/// # Errors
///
/// `FirmanteNoEncontrado` si el psicólogo ya no está registrado o se está eliminando (y
/// entonces no se guarda nada), y los de [`RepositorioRealizarRevision::realizar_revision`].
pub struct RealizarRevision<R, F> {
    repo: R,
    firmas: F,
}

impl<R: RepositorioRealizarRevision, F: RepositorioFirma> RealizarRevision<R, F> {
    pub fn new(repo: R, firmas: F) -> Self {
        Self { repo, firmas }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<OutputData, RespuestaError> {
        let respuesta_id = RespuestaID::new(&in_.respuesta_id)?;
        let firma = self
            .firmas
            .obtener_firma(in_.psicologo_id)
            .await?
            .ok_or(RespuestaError::FirmanteNoEncontrado)?;

        let revision = RevisionRealizada {
            respuesta_id,
            evaluacion_id: in_.evaluacion_id,
            examenes: in_
                .examenes
                .into_iter()
                .map(|ex| ExamenRevision {
                    examen_id: ex.examen_id,
                    observacion: ex.observacion,
                })
                .collect(),
            resultado: in_.resultado,
            revisado_por: in_.psicologo_id,
            firma,
            fecha: in_.ahora,
        };
        self.repo.realizar_revision(&revision).await?;
        Ok(OutputData {
            firma: revision.firma,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    const HOJA: &str = "0b9f1a52-3d4e-4f60-8a7b-9c0d1e2f3a4b";
    const PSICOLOGA: &str = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";

    fn firma_de_maria() -> Firma {
        Firma::new("Maria", "Garcia", "Lopez", "CPsP-123")
    }

    /// Repositorio de hojas y de firmas a la vez: conoce a `PSICOLOGA` (si `registrada`) y
    /// guarda quién firmó la última revisión escrita.
    #[derive(Clone, Default)]
    struct Falso {
        registrada: bool,
        guardada: Arc<Mutex<Option<(PsicologoID, Firma)>>>,
    }

    impl RepositorioFirma for Falso {
        async fn obtener_firma(&self, id: PsicologoID) -> Result<Option<Firma>, RespuestaError> {
            let conocida = self.registrada && id.to_string() == PSICOLOGA;
            Ok(conocida.then(firma_de_maria))
        }
    }

    impl RepositorioRealizarRevision for Falso {
        async fn realizar_revision(
            &self,
            revision: &RevisionRealizada,
        ) -> Result<(), RespuestaError> {
            *self.guardada.lock().unwrap() = Some((revision.revisado_por, revision.firma.clone()));
            Ok(())
        }
    }

    async fn revisar(falso: &Falso) -> Result<OutputData, RespuestaError> {
        RealizarRevision::new(falso.clone(), falso.clone())
            .ejecutar(InputData {
                respuesta_id: HOJA.to_string(),
                evaluacion_id: "2cf52b7a-0ee3-43a9-9b89-4a8baaa22250".to_string(),
                resultado: "apto".to_string(),
                examenes: Vec::new(),
                psicologo_id: PsicologoID::new(PSICOLOGA).unwrap(),
                ahora: DateTime::parse_from_rfc3339("2026-10-16T10:00:00-05:00").unwrap(),
            })
            .await
    }

    #[tokio::test]
    async fn el_psicologo_que_califica_firma_con_su_nombre_y_su_colegiatura() {
        let falso = Falso {
            registrada: true,
            ..Falso::default()
        };

        let salida = revisar(&falso).await.unwrap();

        assert_eq!(salida.firma, firma_de_maria());
        assert_eq!(
            *falso.guardada.lock().unwrap(),
            Some((PsicologoID::new(PSICOLOGA).unwrap(), firma_de_maria()))
        );
    }

    #[tokio::test]
    async fn un_psicologo_que_ya_no_esta_registrado_no_califica() {
        let falso = Falso::default();

        let resultado = revisar(&falso).await;

        assert!(matches!(
            resultado,
            Err(RespuestaError::FirmanteNoEncontrado)
        ));
        assert!(falso.guardada.lock().unwrap().is_none());
    }
}
