use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::entity::examen::Examen;
use crate::respuesta::domain::entity::pregunta::Pregunta;
use crate::respuesta::domain::entity::respuesta::Respuesta;
use crate::respuesta::domain::entity::revision::Firma;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::{RepositorioFirma, RepositorioObtenerRevisionPorId};
use std::collections::HashMap;

pub struct InputData {
    pub revision_id: String,
}

pub struct OutputData {
    pub id: String,
    pub postulante_id: String,
    pub resultado: String,
    pub revision: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    /// La firma del psicólogo que calificó; `None` si calificó un admin.
    pub firma: Option<Firma>,
    pub fecha_revision: Option<String>,
    pub evaluacion: OutputEvaluacion,
}

pub struct OutputEvaluacion {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<OutputExamen>,
}

impl From<Evaluacion> for OutputEvaluacion {
    fn from(evaluacion: Evaluacion) -> Self {
        Self {
            id: evaluacion.id.to_string(),
            nombre: evaluacion.nombre,
            descripcion: evaluacion.descripcion,
            examenes: evaluacion
                .examenes
                .into_iter()
                .map(|examen| examen.into())
                .collect(),
        }
    }
}

pub struct OutputExamen {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<OutputPregunta>,
    pub puntos_obtenidos: i64,
    pub observacion: String,
}

impl From<Examen> for OutputExamen {
    fn from(examen: Examen) -> Self {
        let puntos_obtenidos = examen.puntos_obtenidos();
        Self {
            id: examen.id.to_string(),
            titulo: examen.titulo,
            descripcion: examen.descripcion,
            instrucciones: examen.instrucciones,
            preguntas: examen
                .preguntas
                .into_iter()
                .map(|pregunta| pregunta.into())
                .collect(),
            puntos_obtenidos,
            observacion: examen.observacion,
        }
    }
}

pub struct OutputPregunta {
    pub id: String,
    pub contenido: String,
    pub tipo_de_pregunta: String,
    pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,
    pub respuestas: Option<Vec<String>>,
    pub puntos: i64,
}

impl From<Pregunta> for OutputPregunta {
    fn from(pregunta: Pregunta) -> Self {
        Self {
            id: pregunta.id.to_string(),
            contenido: pregunta.contenido,
            tipo_de_pregunta: pregunta.tipo_de_pregunta.to_string(),
            imagen_ref: pregunta.imagen_ref,
            alternativas: pregunta.alternativas,
            respuestas: pregunta.respuestas,
            puntos: pregunta.puntos,
        }
    }
}

/// Una hoja finalizada con su calificación y la firma de quien la calificó (no de quien la
/// consulta).
pub struct ObtenerRevisionPorId<R, F> {
    repo: R,
    firmas: F,
}

impl<R: RepositorioObtenerRevisionPorId, F: RepositorioFirma> ObtenerRevisionPorId<R, F> {
    pub fn new(repo: R, firmas: F) -> Self {
        Self { repo, firmas }
    }

    pub async fn ejecutar(&self, input: InputData) -> Result<OutputData, RespuestaError> {
        let respuesta_id = RespuestaID::new(&input.revision_id)?;
        let respuesta = self.repo.obtener_revision_por_id(&respuesta_id).await?;
        let firma = self.firma(&respuesta).await?;

        Ok(OutputData {
            id: respuesta.id.to_string(),
            postulante_id: respuesta.postulante.to_string(),
            resultado: respuesta.resultado,
            revision: respuesta.revision.to_string(),
            fecha_tiempo_inicio: respuesta.fecha_tiempo_inicio,
            fecha_tiempo_fin: respuesta.fecha_tiempo_fin,
            firma,
            fecha_revision: respuesta.fecha_revision,
            evaluacion: respuesta.evaluacion.into(),
        })
    }

    /// La firma guardada en la hoja. Las hojas calificadas antes de guardar las firmas solo
    /// tienen el id del revisor: para ellas se usa la del psicólogo, mientras exista.
    async fn firma(&self, respuesta: &Respuesta) -> Result<Option<Firma>, RespuestaError> {
        if let Some(firma) = &respuesta.firma {
            return Ok(Some(firma.clone()));
        }
        match respuesta.revisado_por.as_deref().map(PsicologoID::new) {
            Some(Ok(id)) => self.firmas.obtener_firma(id).await,
            _ => Ok(None),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluacion::value_object::id::EvaluacionID;
    use crate::postulante::domain::value_object::id::PostulanteID;
    use crate::respuesta::domain::entity::respuesta::{Estado, Revision};

    const HOJA: &str = "0b9f1a52-3d4e-4f60-8a7b-9c0d1e2f3a4b";
    const PSICOLOGA: &str = "a1b2c3d4-e5f6-7890-abcd-ef1234567890";

    /// Una hoja calificada por `PSICOLOGA` (con la firma `guardada`) y el registro actual de
    /// esa psicóloga (`actual`, `None` si ya se eliminó).
    struct Falso {
        guardada: Option<Firma>,
        actual: Option<Firma>,
    }

    impl RepositorioObtenerRevisionPorId for &Falso {
        async fn obtener_revision_por_id(
            &self,
            respuesta_id: &RespuestaID,
        ) -> Result<Respuesta, RespuestaError> {
            Ok(Respuesta {
                id: *respuesta_id,
                estado: Estado::Finalizado,
                fecha_tiempo_inicio: String::new(),
                fecha_tiempo_fin: String::new(),
                evaluacion: Evaluacion {
                    id: EvaluacionID::new("2cf52b7a-0ee3-43a9-9b89-4a8baaa22250").unwrap(),
                    nombre: String::new(),
                    descripcion: String::new(),
                    examenes: Vec::new(),
                },
                postulante: PostulanteID::new("e17439e0-79e1-47e3-b5f9-5b54367fa290").unwrap(),
                revision: Revision::Finalizada,
                resultado: "apto".to_string(),
                revisado_por: Some(PSICOLOGA.to_string()),
                firma: self.guardada.clone(),
                fecha_revision: None,
            })
        }
    }

    impl RepositorioFirma for &Falso {
        async fn obtener_firma(&self, id: PsicologoID) -> Result<Option<Firma>, RespuestaError> {
            assert_eq!(id.to_string(), PSICOLOGA);
            Ok(self.actual.clone())
        }
    }

    async fn obtener(falso: &Falso) -> Option<Firma> {
        ObtenerRevisionPorId::new(falso, falso)
            .ejecutar(InputData {
                revision_id: HOJA.to_string(),
            })
            .await
            .unwrap()
            .firma
    }

    #[tokio::test]
    async fn la_firma_es_la_guardada_al_calificar_aunque_el_psicologo_ya_no_exista() {
        let guardada = Firma::new("Maria", "Garcia", "Lopez", "CPsP-123");
        let falso = Falso {
            guardada: Some(guardada.clone()),
            actual: None,
        };
        assert_eq!(obtener(&falso).await, Some(guardada));
    }

    #[tokio::test]
    async fn una_hoja_calificada_antes_de_guardar_firmas_usa_la_del_psicologo() {
        let actual = Firma::new("Maria", "Garcia", "Lopez", "CPsP-123");
        let falso = Falso {
            guardada: None,
            actual: Some(actual.clone()),
        };
        assert_eq!(obtener(&falso).await, Some(actual));
    }
}
