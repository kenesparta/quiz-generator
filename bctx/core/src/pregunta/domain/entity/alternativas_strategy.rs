use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;
use crate::pregunta::domain::entity::tipo_pregunta_strategy::TipoPreguntaStrategy;

pub struct PreguntaAlternativasStrategy;

impl TipoPreguntaStrategy for PreguntaAlternativasStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError> {
        match alternativas {
            None => Err(PreguntaError::AlternativasNoExisten),
            Some(a) if a.is_empty() => Err(PreguntaError::AlternativasVacias),
            Some(a) => Ok(Some(a)),
        }
    }

    fn ajustar_puntos(
        &self,
        puntos: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
        match puntos {
            None => Err(PreguntaError::AlternativasNoExisten),
            Some(p) if p.is_empty() => Err(PreguntaError::PuntosVacios),
            Some(p) => Ok(Some(p)),
        }
    }
}