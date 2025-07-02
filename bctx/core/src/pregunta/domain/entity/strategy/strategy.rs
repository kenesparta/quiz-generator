use crate::pregunta::domain::entity::strategy::alternativa_peso::PreguntaAlternativasConPesoStrategy;
use crate::pregunta::domain::entity::strategy::alternativa_unica::PreguntaAlternativaRespuestaUnicaStrategy;
use crate::pregunta::domain::entity::strategy::libre::PreguntaLibreStrategy;
use crate::pregunta::domain::entity::strategy::si_no::PreguntaSiNoStrategy;
use crate::pregunta::domain::entity::strategy::sola_respuesta::PreguntaSolaRespuestaStrategy;
use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;

pub trait TipoPreguntaStrategy {
    fn ajustar_alternativas(
        &self,
        alternativas: &HashMap<String, String>,
    ) -> Result<(), PreguntaError> {
        match parse_map(alternativas)? {
            None => Err(PreguntaError::AlternativasNoExisten),
            Some(a) => {
                if a.is_empty() {
                    return Err(PreguntaError::AlternativasVacias);
                }
                Ok(())
            }
        }
    }

    fn ajustar_puntaje(&self, puntaje: &HashMap<String, u32>) -> Result<(), PreguntaError> {
        match parse_map(puntaje)? {
            None => Err(PreguntaError::PuntajeNoExiste),
            Some(a) => {
                if a.is_empty() {
                    return Err(PreguntaError::PuntajeVacio);
                }
                Ok(())
            }
        }
    }

    fn verificar_consistencia(
        &self,
        alternativas: &HashMap<String, String>,
        puntaje: &HashMap<String, u32>,
    ) -> Result<(), PreguntaError> {
        let missing_key = puntaje.keys().find(|k| !alternativas.contains_key(*k));
        match missing_key {
            Some(_) => Err(PreguntaError::PuntajeNoCoincideConAlternativa),
            None => Ok(()),
        }
    }

    fn verify(
        &self,
        alternativas: &HashMap<String, String>,
        puntaje: &HashMap<String, u32>,
    ) -> Result<(), PreguntaError> {
        self.ajustar_alternativas(alternativas)?;
        self.ajustar_puntaje(puntaje)?;
        self.verificar_consistencia(alternativas, puntaje)
    }
}

pub fn parse_map<V>(
    map: &HashMap<String, V>,
) -> Result<Option<HashMap<Alternativa, V>>, PreguntaError>
where
    V: Clone,
{
    if map.is_empty() {
        return Ok(None);
    }

    let results: Result<Vec<(Alternativa, V)>, AlternativaError> = map
        .iter()
        .map(|(key, value)| key.parse::<Alternativa>().map(|alt| (alt, value.clone())))
        .collect();

    match results {
        Ok(pairs) => Ok(Some(pairs.into_iter().collect())),
        Err(err) => Err(PreguntaError::PreguntaAlternativaError(err)),
    }
}

pub fn strategy_selection(tipo: &TipoPregunta) -> Box<dyn TipoPreguntaStrategy> {
    match tipo {
        TipoPregunta::AlternativaUnica => Box::new(PreguntaAlternativaRespuestaUnicaStrategy),
        TipoPregunta::AlternativaConPeso => Box::new(PreguntaAlternativasConPesoStrategy),
        TipoPregunta::Libre => Box::new(PreguntaLibreStrategy),
        TipoPregunta::SolaRespuesta => Box::new(PreguntaSolaRespuestaStrategy),
        TipoPregunta::SioNo => Box::new(PreguntaSiNoStrategy),
    }
}
