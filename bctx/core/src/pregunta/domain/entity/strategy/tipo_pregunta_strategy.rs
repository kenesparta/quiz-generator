use crate::pregunta::domain::entity::strategy::alternativa_unica::PreguntaAlternativaRespuestaUnicaStrategy;
use crate::pregunta::domain::entity::strategy::libre::PreguntaLibreStrategy;
use crate::pregunta::domain::entity::strategy::si_no::PreguntaSiNoStrategy;
use crate::pregunta::domain::entity::strategy::sola_respuesta::PreguntaSolaRespuestaStrategy;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;

pub trait TipoPreguntaStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: Option<HashMap<Alternativa, String>>,
    ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError>;

    fn ajustar_puntaje(
        &self,
        puntaje: Option<HashMap<Alternativa, u32>>,
    ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError>;

    fn verificar_consistencia(
        &self,
        alternativas: &Option<HashMap<Alternativa, String>>,
        puntaje: &Option<HashMap<Alternativa, u32>>,
    ) -> Result<(), PreguntaError> {
        let alternativas = match alternativas {
            None => return Ok(()),
            Some(alt) => alt,
        };

        let puntaje = match puntaje {
            None => return Err(PreguntaError::RespuestaNoExiste),
            Some(pts) => pts,
        };

        let exists_in_puntaje = alternativas.keys().any(|k| puntaje.contains_key(k));
        if !exists_in_puntaje {
            return Err(PreguntaError::RespuestaNoExiste);
        }

        Ok(())
    }
}

pub fn get_strategy(tipo: &TipoPregunta) -> Box<dyn TipoPreguntaStrategy> {
    match tipo {
        TipoPregunta::AlternativaUnica => Box::new(PreguntaAlternativaRespuestaUnicaStrategy),
        TipoPregunta::AlternativaConPeso => Box::new(PreguntaAlternativaRespuestaUnicaStrategy),
        TipoPregunta::Libre => Box::new(PreguntaLibreStrategy),
        TipoPregunta::SolaRespuesta => Box::new(PreguntaSolaRespuestaStrategy),
        TipoPregunta::SioNo => Box::new(PreguntaSiNoStrategy),
    }
}
