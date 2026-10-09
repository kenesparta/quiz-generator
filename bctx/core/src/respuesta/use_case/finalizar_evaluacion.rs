use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use crate::respuesta::provider::repositorio::RepositorioEstadoRespuesta;
use chrono::{DateTime, FixedOffset};

pub struct InputData {
    pub id: String,
    /// Dueño de la hoja, tomado del token.
    pub postulante_id: String,
    pub ahora: DateTime<FixedOffset>,
}

/// Finaliza el examen: `EnProceso` -> `Finalizado` y registra la hora de fin.
///
/// Es idempotente (finalizar dos veces no hace nada) y atómico: una vez finalizado no se
/// aceptan más respuestas, así que los puntos que se leen después no cambian. Falla con
/// `RespuestaNoEncontrada` si la hoja no es del postulante y con `EvaluacionNoEstaEnProceso`
/// si todavía no empezó.
pub struct FinalizarEvaluacion<R> {
    repositorio: R,
}

impl<R: RepositorioEstadoRespuesta> FinalizarEvaluacion<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        let id = RespuestaID::new(&in_.id)?;
        let postulante_id = PostulanteID::new(&in_.postulante_id)?;

        let finalizado = self
            .repositorio
            .transicionar(
                &id,
                &postulante_id,
                Estado::EnProceso,
                Estado::Finalizado,
                in_.ahora,
            )
            .await?;
        if finalizado {
            return Ok(());
        }

        match self.repositorio.obtener_estado(&id, &postulante_id).await? {
            None => Err(RespuestaError::RespuestaNoEncontrada),
            Some(Estado::Finalizado) => Ok(()),
            Some(Estado::Creado) => Err(RespuestaError::EvaluacionNoEstaEnProceso),
            Some(Estado::EnProceso) => Err(RespuestaError::TransicionNoAplicada),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::respuesta::use_case::transicion_estado::{DUENO, HOJA, HojaEnMemoria, OTRO, ahora};

    async fn finalizar(hoja: &HojaEnMemoria, postulante_id: &str) -> Result<(), RespuestaError> {
        FinalizarEvaluacion::new(hoja.clone())
            .ejecutar(InputData {
                id: HOJA.to_string(),
                postulante_id: postulante_id.to_string(),
                ahora: ahora(),
            })
            .await
    }

    #[tokio::test]
    async fn finalizar_pasa_a_finalizado_y_registra_la_hora() {
        let hoja = HojaEnMemoria::en(Estado::EnProceso);
        finalizar(&hoja, DUENO).await.unwrap();
        assert_eq!(hoja.estado(), Estado::Finalizado);
        assert_eq!(hoja.fecha(), Some(ahora()));
    }

    #[tokio::test]
    async fn finalizar_dos_veces_no_cambia_la_hora() {
        let hoja = HojaEnMemoria::en(Estado::Finalizado);
        finalizar(&hoja, DUENO).await.unwrap();
        assert_eq!(hoja.fecha(), None);
    }

    #[tokio::test]
    async fn no_se_puede_finalizar_sin_empezar() {
        let hoja = HojaEnMemoria::en(Estado::Creado);
        let resultado = finalizar(&hoja, DUENO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::EvaluacionNoEstaEnProceso)
        ));
        assert_eq!(hoja.estado(), Estado::Creado);
    }

    #[tokio::test]
    async fn otro_postulante_no_puede_finalizar_la_hoja() {
        let hoja = HojaEnMemoria::en(Estado::EnProceso);
        let resultado = finalizar(&hoja, OTRO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::RespuestaNoEncontrada)
        ));
        assert_eq!(hoja.estado(), Estado::EnProceso);
    }
}
