use super::strategy::{TipoPreguntaStrategy, parse_map};
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;

pub struct PreguntaSiNoStrategy;

impl TipoPreguntaStrategy for PreguntaSiNoStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: &HashMap<String, String>,
    ) -> Result<(), PreguntaError> {
        match parse_map(alternativas)? {
            None => Ok(()),
            Some(alt) => {
                let valid_keys = vec![Alternativa::Si, Alternativa::No];
                let filtered: HashMap<_, _> = alt
                    .into_iter()
                    .filter(|(key, _)| valid_keys.contains(key))
                    .collect();

                if !filtered.is_empty() {
                    return Err(PreguntaError::AlternativaNoAjustada);
                }

                Ok(())
            }
        }
    }

    fn ajustar_puntaje(&self, puntaje: &HashMap<String, u32>) -> Result<(), PreguntaError> {
        match parse_map(puntaje)? {
            None => Ok(()),
            Some(pts) => {
                let valid_keys = vec![Alternativa::Si, Alternativa::No];
                let filtered: HashMap<_, _> = pts
                    .into_iter()
                    .filter(|(key, _)| valid_keys.contains(key))
                    .collect();

                if !filtered.is_empty() {
                    return Err(PreguntaError::PuntajeNoAjustado);
                }

                Ok(())
            }
        }
    }
}
