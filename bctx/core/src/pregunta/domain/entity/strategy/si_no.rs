use super::strategy::{TipoPreguntaStrategy, parse_map};
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;

fn ajustar<T>(p: &HashMap<String, T>) -> Result<(), PreguntaError>
where
    T: Clone,
{
    match parse_map(p)? {
        None => Ok(()),
        Some(alt) => {
            let valid_keys = vec![Alternativa::Si, Alternativa::No];
            let filtered: HashMap<_, _> = alt
                .into_iter()
                .filter(|(key, _)| valid_keys.contains(key))
                .collect();

            if filtered.len() != 2 {
                return Err(PreguntaError::AlternativaNoAjustada);
            }

            if !filtered.contains_key(&Alternativa::Si) || !filtered.contains_key(&Alternativa::No)
            {
                return Err(PreguntaError::AlternativaNoAjustada);
            }

            Ok(())
        }
    }
}

pub struct PreguntaSiNoStrategy;

impl TipoPreguntaStrategy for PreguntaSiNoStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: &HashMap<String, String>,
    ) -> Result<(), PreguntaError> {
        ajustar(alternativas)
    }

    fn ajustar_puntaje(&self, puntaje: &HashMap<String, u32>) -> Result<(), PreguntaError> {
        ajustar(puntaje)
    }
}
