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
            None => return Err(PreguntaError::AlternativasNoExisten),
            Some(alternativa) => alternativa,
        };

        let puntaje = match puntaje {
            None => return Err(PreguntaError::PuntajeNoExiste),
            Some(puntaje) => puntaje,
        };

        let missing_key = puntaje.keys().find(|k| !alternativas.contains_key(k));
        match missing_key {
            Some(_) => Err(PreguntaError::PuntajeNoCoincideConAlternativa),
            None => Ok(()),
        }
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct MockStrategy;

    impl TipoPreguntaStrategy for MockStrategy {
        fn ajustar_alternativas(
            &self,
            _alternativas: Option<HashMap<Alternativa, String>>,
        ) -> Result<Option<HashMap<Alternativa, String>>, PreguntaError> {
            todo!()
        }

        fn ajustar_puntaje(
            &self,
            _puntaje: Option<HashMap<Alternativa, u32>>,
        ) -> Result<Option<HashMap<Alternativa, u32>>, PreguntaError> {
            todo!()
        }
    }

    #[test]
    fn test_verificar_consistencia_alternativas_none() {
        let strategy = MockStrategy;
        let alternativas: Option<HashMap<Alternativa, String>> = None;
        let puntaje: Option<HashMap<Alternativa, u32>> = Some(HashMap::new());

        let result = strategy.verificar_consistencia(&alternativas, &puntaje);
        assert!(result.is_err());
    }

    #[test]
    fn test_verificar_consistencia_puntaje_none() {
        let strategy = MockStrategy;
        let mut alternativas = HashMap::new();
        alternativas.insert(Alternativa::A, "Opción A".to_string());
        let puntaje: Option<HashMap<Alternativa, u32>> = None;

        let result = strategy.verificar_consistencia(&Some(alternativas), &puntaje);

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PreguntaError::PuntajeNoExiste
        ));
    }

    #[test]
    fn test_verificar_consistencia_no_matching_keys() {
        // Arrange
        let strategy = MockStrategy;
        let mut alternativas = HashMap::new();
        alternativas.insert(Alternativa::A, "Opción A".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert(Alternativa::B, 10);

        let result = strategy.verificar_consistencia(&Some(alternativas), &Some(puntaje));

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PreguntaError::PuntajeNoCoincideConAlternativa
        ));
    }

    #[test]
    fn test_verificar_consistencia_matching_keys() {
        let strategy = MockStrategy;
        let mut alternativas = HashMap::new();
        alternativas.insert(Alternativa::A, "Opción A".to_string());
        alternativas.insert(Alternativa::B, "Opción B".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert(Alternativa::C, 10);
        puntaje.insert(Alternativa::A, 5);

        let result = strategy.verificar_consistencia(&Some(alternativas), &Some(puntaje));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PreguntaError::PuntajeNoCoincideConAlternativa
        ));
    }

    #[test]
    fn test_verificar_consistencia_empty_alternativas() {
        let strategy = MockStrategy;
        let alternativas: Option<HashMap<Alternativa, String>> = None;

        let mut puntaje = HashMap::new();
        puntaje.insert(Alternativa::A, 10);

        let result = strategy.verificar_consistencia(&alternativas, &Some(puntaje));
        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            PreguntaError::AlternativasNoExisten
        ));
    }
}
