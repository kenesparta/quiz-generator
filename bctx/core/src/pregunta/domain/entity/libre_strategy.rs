use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;
use crate::pregunta::domain::entity::tipo_pregunta_strategy::TipoPreguntaStrategy;

pub struct PreguntaLibreStrategy;

impl TipoPreguntaStrategy for PreguntaLibreStrategy {
    fn ajustar_alternativas(
        &self,
        _alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError> {
        Ok(None)
    }

    fn ajustar_puntos(
        &self,
        _puntos: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
        Ok(None)
    }
}