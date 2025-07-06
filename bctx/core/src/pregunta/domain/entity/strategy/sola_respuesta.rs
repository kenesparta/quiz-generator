use crate::pregunta::domain::entity::strategy::strategy::TipoPreguntaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use std::collections::HashMap;

pub struct PreguntaSolaRespuestaStrategy;

impl TipoPreguntaStrategy for PreguntaSolaRespuestaStrategy {
    fn ajustar_alternativas(
        &self,
        _alternativas: &HashMap<String, String>,
    ) -> Result<(), PreguntaError> {
        Ok(())
    }

    fn ajustar_puntaje(&self, _puntaje: &HashMap<String, u32>) -> Result<(), PreguntaError> {
        Ok(())
    }

    fn verificar_consistencia(
        &self,
        _alternativas: &HashMap<String, String>,
        puntaje: &HashMap<String, u32>,
    ) -> Result<(), PreguntaError> {
        if puntaje.len() == 0 {
            return Err(PreguntaError::PuntajeNoExiste);
        }

        if puntaje.len() > 1 {
            return Err(PreguntaError::DebeTenerUnaSolaRespuesta);
        }

        Ok(())
    }
}
