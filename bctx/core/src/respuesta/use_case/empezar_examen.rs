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

/// Inicia el examen: `Creado` -> `EnProceso` y registra la hora de inicio.
///
/// Es idempotente (empezar un examen en proceso no hace nada) y atómico: si dos peticiones
/// llegan a la vez, solo una registra la hora. Falla con `RespuestaNoEncontrada` si la hoja no
/// es del postulante y con `EvaluacionFinalizada` si ya terminó.
pub struct EmpezarExamen<R> {
    repositorio: R,
}

impl<R: RepositorioEstadoRespuesta> EmpezarExamen<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), RespuestaError> {
        let id = RespuestaID::new(&in_.id)?;
        let postulante_id = PostulanteID::new(&in_.postulante_id)?;

        let empezado = self
            .repositorio
            .transicionar(
                &id,
                &postulante_id,
                Estado::Creado,
                Estado::EnProceso,
                in_.ahora,
            )
            .await?;
        if empezado {
            return Ok(());
        }

        match self.repositorio.obtener_estado(&id, &postulante_id).await? {
            None => Err(RespuestaError::RespuestaNoEncontrada),
            Some(Estado::EnProceso) => Ok(()),
            Some(Estado::Finalizado) => Err(RespuestaError::EvaluacionFinalizada),
            Some(Estado::Creado) => Err(RespuestaError::TransicionNoAplicada),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::respuesta::use_case::transicion_estado::{DUENO, HOJA, HojaEnMemoria, OTRO, ahora};

    async fn empezar(hoja: &HojaEnMemoria, postulante_id: &str) -> Result<(), RespuestaError> {
        EmpezarExamen::new(hoja.clone())
            .ejecutar(InputData {
                id: HOJA.to_string(),
                postulante_id: postulante_id.to_string(),
                ahora: ahora(),
            })
            .await
    }

    #[tokio::test]
    async fn empezar_pasa_a_en_proceso_y_registra_la_hora() {
        let hoja = HojaEnMemoria::en(Estado::Creado);
        empezar(&hoja, DUENO).await.unwrap();
        assert_eq!(hoja.estado(), Estado::EnProceso);
        assert_eq!(hoja.fecha(), Some(ahora()));
    }

    #[tokio::test]
    async fn empezar_un_examen_en_proceso_no_cambia_la_hora() {
        let hoja = HojaEnMemoria::en(Estado::EnProceso);
        empezar(&hoja, DUENO).await.unwrap();
        assert_eq!(hoja.fecha(), None);
    }

    #[tokio::test]
    async fn no_se_puede_empezar_un_examen_finalizado() {
        let hoja = HojaEnMemoria::en(Estado::Finalizado);
        let resultado = empezar(&hoja, DUENO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::EvaluacionFinalizada)
        ));
    }

    #[tokio::test]
    async fn otro_postulante_no_puede_empezar_la_hoja() {
        let hoja = HojaEnMemoria::en(Estado::Creado);
        let resultado = empezar(&hoja, OTRO).await;
        assert!(matches!(
            resultado,
            Err(RespuestaError::RespuestaNoEncontrada)
        ));
        assert_eq!(hoja.estado(), Estado::Creado);
    }

    #[tokio::test]
    async fn si_otra_peticion_lo_empezo_primero_es_idempotente() {
        let mut hoja = HojaEnMemoria::en(Estado::Creado);
        hoja.perder_carrera = Some(Estado::EnProceso);
        empezar(&hoja, DUENO).await.unwrap();
        assert_eq!(hoja.fecha(), None);
    }
}
