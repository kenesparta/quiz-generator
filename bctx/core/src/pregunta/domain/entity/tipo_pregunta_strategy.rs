use crate::pregunta::domain::entity::alternativas_strategy::PreguntaAlternativasStrategy;
use crate::pregunta::domain::entity::libre_strategy::PreguntaLibreStrategy;
use crate::pregunta::domain::entity::si_no_strategy::PreguntaSiNoStrategy;
use crate::pregunta::domain::entity::sola_respuesta_strategy::PreguntaSolaRespuestaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;

pub trait TipoPreguntaStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError>;

    fn ajustar_puntos(
        &self,
        puntos: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError>;

    fn verificar_consistencia(
        &self,
        alternativas: &Option<HashMap<Alternativa, String>>,
        puntos: &Option<HashMap<Alternativa, u32>>,
    ) -> Result<(), PreguntaError> {
        let alternativas = match alternativas {
            None => return Ok(()),
            Some(alt) => alt,
        };

        let puntos = match puntos {
            None => return Err(PreguntaError::RespuestaNoExiste),
            Some(pts) => pts,
        };

        let exists_in_puntos = alternativas.keys().any(|k| puntos.contains_key(k));
        if !exists_in_puntos {
            return Err(PreguntaError::RespuestaNoExiste);
        }

        Ok(())
    }
}

pub fn get_strategy(tipo: &TipoPregunta) -> Box<dyn TipoPreguntaStrategy> {
    match tipo {
        TipoPregunta::Alternativas => Box::new(PreguntaAlternativasStrategy),
        TipoPregunta::Libre => Box::new(PreguntaLibreStrategy),
        TipoPregunta::SolaRespuesta => Box::new(PreguntaSolaRespuestaStrategy),
        TipoPregunta::SioNo => Box::new(PreguntaSiNoStrategy),
    }
}
