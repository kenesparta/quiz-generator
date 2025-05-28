use crate::pregunta::domain::entity::strategy::tipo_pregunta_strategy::TipoPreguntaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;

pub struct PreguntaAlternativasConPesoStrategy;

impl TipoPreguntaStrategy for PreguntaAlternativasConPesoStrategy {
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

    fn ajustar_puntaje(
        &self,
        puntaje: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
        match puntaje {
            None => Err(PreguntaError::AlternativasNoExisten),
            Some(p) if p.is_empty() => Err(PreguntaError::PuntajeVacio),
            Some(p) => Ok(Some(p)),
        }
    }
}
