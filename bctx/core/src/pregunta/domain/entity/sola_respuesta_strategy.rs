use crate::pregunta::domain::entity::tipo_pregunta_strategy::TipoPreguntaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use std::collections::HashMap;

pub struct PreguntaSolaRespuestaStrategy;

impl TipoPreguntaStrategy for PreguntaSolaRespuestaStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError> {
        match alternativas {
            None => Ok(None),
            Some(alt) if alt.is_empty() => Ok(None),
            Some(alt) => {
                let entry = alt.into_iter().next().unwrap();
                let mut new_map = HashMap::new();
                new_map.insert(entry.0, entry.1);
                Ok(Some(new_map))
            }
        }
    }

    fn ajustar_puntos(
        &self,
        puntos: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
        match puntos {
            None => Ok(None),
            Some(pts) if pts.is_empty() => Ok(None),
            Some(pts) => {
                let entry = pts.into_iter().next().unwrap();
                let mut new_map = HashMap::new();
                new_map.insert(entry.0, entry.1);
                Ok(Some(new_map))
            }
        }
    }
}
