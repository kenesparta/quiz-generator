use crate::pregunta::domain::entity::strategy::strategy::TipoPreguntaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use std::collections::HashMap;

pub struct PreguntaLibreStrategy;

impl TipoPreguntaStrategy for PreguntaLibreStrategy {
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
        _puntaje: &HashMap<String, u32>,
    ) -> Result<(), PreguntaError> {
        Ok(())
    }
}
