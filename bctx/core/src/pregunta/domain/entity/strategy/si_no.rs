use super::tipo_pregunta_strategy::TipoPreguntaStrategy;
use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;

pub struct PreguntaSiNoStrategy;

impl TipoPreguntaStrategy for PreguntaSiNoStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError> {
        match alternativas {
            None => Ok(None),
            Some(alt) => {
                let valid_keys = vec![Alternativa::Si, Alternativa::No];
                let filtered: HashMap<_, _> = alt
                    .into_iter()
                    .filter(|(key, _)| valid_keys.contains(key))
                    .collect();

                if filtered.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(filtered))
                }
            }
        }
    }

    fn ajustar_puntaje(
        &self,
        puntaje: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
        match puntaje {
            None => Ok(None),
            Some(pts) => {
                let valid_keys = vec![Alternativa::Si, Alternativa::No];
                let filtered: HashMap<_, _> = pts
                    .into_iter()
                    .filter(|(key, _)| valid_keys.contains(key))
                    .collect();

                if filtered.is_empty() {
                    Ok(None)
                } else {
                    Ok(Some(filtered))
                }
            }
        }
    }
}
