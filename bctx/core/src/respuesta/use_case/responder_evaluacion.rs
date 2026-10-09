use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::pregunta::corregir_respuesta;
use crate::respuesta::domain::entity::respuesta::{Estado, RespuestaEvaluacion};
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioRespuestaEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    /// Dueño de la hoja, tomado del token: nunca del cuerpo ni de la ruta.
    pub postulante_id: String,
    pub examen_id: String,
    pub pregunta_id: String,
    pub respuestas: Vec<String>,
}

/// Guarda la contestación de una pregunta con sus puntos.
///
/// Solo el dueño de la hoja puede contestar, y solo mientras está en proceso: la comprobación y
/// la escritura usan el mismo filtro (dueño + estado), así que una respuesta que llega tras
/// finalizar se rechaza aunque compita con la finalización.
pub struct ResponderEvaluacion<RepoErr> {
    repositorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>,
}

impl<RepoErr> ResponderEvaluacion<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioRespuestaEscritura<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), RespuestaError> for ResponderEvaluacion<RepoErr>
where
    RespuestaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        let mut contestacion = RespuestaEvaluacion {
            id: RespuestaID::new(&in_.id)?,
            postulante_id: PostulanteID::new(&in_.postulante_id)?,
            examen_id: in_.examen_id,
            pregunta_id: in_.pregunta_id,
            puntos: 0,
            respuestas: in_.respuestas,
        };

        let pregunta = self.repositorio.obtener_pregunta(&contestacion).await?;
        match pregunta.estado {
            Estado::EnProceso => {}
            Estado::Finalizado => return Err(RespuestaError::EvaluacionFinalizada),
            Estado::Creado => return Err(RespuestaError::EvaluacionNoEstaEnProceso),
        }

        contestacion.puntos = corregir_respuesta(
            &pregunta.tipo_de_pregunta,
            &contestacion.respuestas,
            &pregunta.alternativas,
            &pregunta.puntaje,
        )?;

        if !self.repositorio.responder_evaluacion(&contestacion).await? {
            // La hoja dejó de estar en proceso entre la lectura y la escritura.
            return Err(RespuestaError::EvaluacionFinalizada);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::evaluacion::value_object::id::EvaluacionID;
    use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
    use crate::respuesta::domain::entity::pregunta::PreguntaACorregir;
    use crate::respuesta::use_case::transicion_estado::{DUENO, HOJA, OTRO};
    use std::collections::HashMap;
    use std::sync::{Arc, Mutex};

    /// Contestaciones guardadas: respuestas y puntos.
    type Guardadas = Arc<Mutex<Vec<(Vec<String>, u32)>>>;

    /// Una hoja de `DUENO` con una pregunta cuya respuesta correcta es "A" (4 puntos).
    #[derive(Clone)]
    struct HojaFalsa {
        estado: Estado,
        /// El estado al escribir, si otra petición lo cambió entre la lectura y la escritura.
        estado_al_escribir: Option<Estado>,
        guardadas: Guardadas,
    }

    impl HojaFalsa {
        fn en(estado: Estado) -> Self {
            Self {
                estado,
                estado_al_escribir: None,
                guardadas: Arc::default(),
            }
        }

        fn es_del_dueno(contestacion: &RespuestaEvaluacion) -> bool {
            contestacion.id.to_string() == HOJA && contestacion.postulante_id.to_string() == DUENO
        }
    }

    #[async_trait]
    impl RepositorioRespuestaEscritura<RespuestaError> for HojaFalsa {
        async fn asignar_evaluacion(
            &self,
            _: &RespuestaID,
            _: EvaluacionID,
            _: PostulanteID,
        ) -> Result<(), RespuestaError> {
            Err(RespuestaError::DatabaseError)
        }

        async fn responder_evaluacion(
            &self,
            contestacion: &RespuestaEvaluacion,
        ) -> Result<bool, RespuestaError> {
            let estado = self.estado_al_escribir.unwrap_or(self.estado);
            if !Self::es_del_dueno(contestacion) || estado != Estado::EnProceso {
                return Ok(false);
            }
            self.guardadas
                .lock()
                .unwrap()
                .push((contestacion.respuestas.clone(), contestacion.puntos));
            Ok(true)
        }

        async fn obtener_pregunta(
            &self,
            contestacion: &RespuestaEvaluacion,
        ) -> Result<PreguntaACorregir, RespuestaError> {
            if !Self::es_del_dueno(contestacion) {
                return Err(RespuestaError::RespuestaNoEncontrada);
            }
            Ok(PreguntaACorregir {
                estado: self.estado,
                tipo_de_pregunta: TipoPregunta::AlternativaUnica,
                alternativas: HashMap::from([
                    ("A".to_string(), "Sí".to_string()),
                    ("B".to_string(), "No".to_string()),
                ]),
                puntaje: HashMap::from([("A".to_string(), 4)]),
            })
        }
    }

    async fn contestar(hoja: &HojaFalsa, postulante_id: &str) -> Result<(), RespuestaError> {
        ResponderEvaluacion::new(Box::new(hoja.clone()))
            .ejecutar(InputData {
                id: HOJA.to_string(),
                postulante_id: postulante_id.to_string(),
                examen_id: "examen".to_string(),
                pregunta_id: "pregunta".to_string(),
                respuestas: vec!["A".to_string()],
            })
            .await
    }

    #[tokio::test]
    async fn guarda_la_respuesta_con_sus_puntos() {
        let hoja = HojaFalsa::en(Estado::EnProceso);
        contestar(&hoja, DUENO).await.unwrap();
        assert_eq!(
            *hoja.guardadas.lock().unwrap(),
            [(vec!["A".to_string()], 4)]
        );
    }

    #[tokio::test]
    async fn una_respuesta_no_valida_no_se_guarda() {
        let hoja = HojaFalsa::en(Estado::EnProceso);
        let resultado = ResponderEvaluacion::new(Box::new(hoja.clone()))
            .ejecutar(InputData {
                id: HOJA.to_string(),
                postulante_id: DUENO.to_string(),
                examen_id: "examen".to_string(),
                pregunta_id: "pregunta".to_string(),
                respuestas: vec!["A".to_string(), "A".to_string()],
            })
            .await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::CantidadDeRespuestasNoValida)
        ));
        assert!(hoja.guardadas.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn otro_postulante_no_puede_contestar_la_hoja() {
        let hoja = HojaFalsa::en(Estado::EnProceso);
        let resultado = contestar(&hoja, OTRO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::RespuestaNoEncontrada)
        ));
        assert!(hoja.guardadas.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn no_se_puede_contestar_antes_de_empezar_ni_despues_de_finalizar() {
        for (estado, esperado) in [
            (Estado::Creado, "no esta en proceso"),
            (Estado::Finalizado, "finalizada"),
        ] {
            let hoja = HojaFalsa::en(estado);
            let error = contestar(&hoja, DUENO).await.unwrap_err();
            assert!(error.to_string().contains(esperado), "{estado}: {error}");
            assert!(hoja.guardadas.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn si_se_finaliza_mientras_contesta_la_respuesta_se_rechaza() {
        let mut hoja = HojaFalsa::en(Estado::EnProceso);
        hoja.estado_al_escribir = Some(Estado::Finalizado);
        let resultado = contestar(&hoja, DUENO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::EvaluacionFinalizada)
        ));
    }
}
