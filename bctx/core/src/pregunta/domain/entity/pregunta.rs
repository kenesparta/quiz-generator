use crate::pregunta::domain::entity::strategy::tipo_pregunta_strategy::get_strategy;
use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PreguntaEntity {
    pub id: PreguntaID,
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: Option<HashMap<Alternativa, String>>,
    pub puntaje: Option<HashMap<Alternativa, u32>>,
}

impl PreguntaEntity {
    pub fn new(
        id: String,
        contenido: String,
        etiqueta: String,
        tipo_de_pregunta: String,
        imagen_ref: Option<String>,
        alternativas: HashMap<String, String>,
        puntaje: HashMap<String, u32>,
    ) -> Result<Self, PreguntaError> {
        let id = PreguntaID::new(&id)?;
        let etiqueta = Etiqueta::from_str(&etiqueta)?;
        let tipo_de_pregunta = TipoPregunta::from_str(&tipo_de_pregunta)?;
        let alternativas = Self::parse_map(alternativas)?;
        let puntaje = Self::parse_map(puntaje)?;

        let strategy = get_strategy(&tipo_de_pregunta);
        let alternativas = strategy.ajustar_alternativas(alternativas)?;
        let puntaje = strategy.ajustar_puntaje(puntaje)?;

        strategy.verificar_consistencia(&alternativas, &puntaje)?;

        Ok(Self {
            id,
            contenido,
            etiqueta,
            tipo_de_pregunta,
            alternativas,
            puntaje,
            imagen_ref,
        })
    }

    fn parse_map<V>(
        map: HashMap<String, V>,
    ) -> Result<Option<HashMap<Alternativa, V>>, PreguntaError>
    where
        V: Clone,
    {
        if map.is_empty() {
            return Ok(None);
        }

        let results: Result<Vec<(Alternativa, V)>, AlternativaError> = map
            .into_iter()
            .map(|(key, value)| key.parse::<Alternativa>().map(|alt| (alt, value)))
            .collect();

        match results {
            Ok(pairs) => Ok(Some(pairs.into_iter().collect())),
            Err(err) => Err(PreguntaError::PreguntaAlternativaError(err)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_create_alternativa_unica_question() {
        let id = "84a2c727-0ded-48e8-967c-e3cc1e35b8bc".to_string();
        let contenido = "¿Cuál es la capital de Francia?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_unica".to_string();
        let imagen_ref = Some("".to_string());

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "París".to_string());
        alternativas.insert("B".to_string(), "Londres".to_string());
        alternativas.insert("C".to_string(), "Madrid".to_string());
        alternativas.insert("D".to_string(), "Roma".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 4);

        let result = PreguntaEntity::new(
            id.clone(),
            contenido.clone(),
            etiqueta.clone(),
            tipo_de_pregunta.clone(),
            imagen_ref.clone(),
            alternativas.clone(),
            puntaje.clone(),
        );

        assert!(result.is_ok());
        let pregunta = result.unwrap();
        assert_eq!(pregunta.id.to_string(), id);
        assert_eq!(pregunta.contenido, contenido);
        assert_eq!(pregunta.etiqueta.to_string(), etiqueta);
        assert_eq!(pregunta.tipo_de_pregunta.to_string(), tipo_de_pregunta);

        let alt_map = pregunta.alternativas.unwrap();
        let punt_map = pregunta.puntaje.unwrap();

        assert!(alt_map.contains_key(&Alternativa::A));
        assert!(alt_map.contains_key(&Alternativa::B));
        assert!(alt_map.contains_key(&Alternativa::C));
        assert!(alt_map.contains_key(&Alternativa::D));

        assert_eq!(alt_map.get(&Alternativa::A).unwrap(), "París");
        assert_eq!(punt_map.get(&Alternativa::A).unwrap(), &4);
    }

    #[test]
    fn test_create_alternativa_peso_question() {
        let id = "84a2c727-0ded-48e8-967c-e3cc1e35b8bc".to_string();
        let contenido = "¿Elige una de las alternativas?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_peso".to_string();
        let imagen_ref = Some("".to_string());

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "alt01".to_string());
        alternativas.insert("B".to_string(), "alt02".to_string());
        alternativas.insert("C".to_string(), "alt03".to_string());
        alternativas.insert("D".to_string(), "alt04".to_string());
        alternativas.insert("E".to_string(), "alt05".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 0);
        puntaje.insert("B".to_string(), 1);
        puntaje.insert("C".to_string(), 2);
        puntaje.insert("D".to_string(), 3);
        puntaje.insert("E".to_string(), 4);

        let result = PreguntaEntity::new(
            id.clone(),
            contenido.clone(),
            etiqueta.clone(),
            tipo_de_pregunta.clone(),
            imagen_ref.clone(),
            alternativas.clone(),
            puntaje.clone(),
        );

        assert!(result.is_ok());
        let pregunta = result.unwrap();
        assert_eq!(pregunta.id.to_string(), id);
        assert_eq!(pregunta.contenido, contenido);
        assert_eq!(pregunta.tipo_de_pregunta.to_string(), tipo_de_pregunta);

        let alt_map = pregunta.alternativas.unwrap();
        let punt_map = pregunta.puntaje.unwrap();

        assert!(alt_map.contains_key(&Alternativa::A));
        assert!(alt_map.contains_key(&Alternativa::B));
        assert!(alt_map.contains_key(&Alternativa::C));
        assert!(alt_map.contains_key(&Alternativa::D));
        assert!(alt_map.contains_key(&Alternativa::E));

        assert_eq!(punt_map.get(&Alternativa::A).unwrap(), &0);
        assert_eq!(punt_map.get(&Alternativa::B).unwrap(), &1);
        assert_eq!(punt_map.get(&Alternativa::C).unwrap(), &2);
        assert_eq!(punt_map.get(&Alternativa::D).unwrap(), &3);
        assert_eq!(punt_map.get(&Alternativa::E).unwrap(), &4);
    }

    #[test]
    fn test_invalid_alternative_key() {
        let id = "84a2c727-0ded-48e8-967c-e3cc1e35b8bc".to_string();
        let contenido = "¿Pregunta de prueba?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_unica".to_string();
        let imagen_ref = None;

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "Opción A".to_string());
        alternativas.insert("X".to_string(), "Opción inválida".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 1);

        let result = PreguntaEntity::new(
            id,
            contenido,
            etiqueta,
            tipo_de_pregunta,
            imagen_ref,
            alternativas,
            puntaje,
        );

        assert!(result.is_err());
        match result {
            Err(PreguntaError::PreguntaAlternativaError(_)) => assert!(true),
            _ => assert!(false, "Expected PreguntaAlternativaError"),
        }
    }

    #[test]
    fn test_invalid_tipo_pregunta() {
        let id = "84a2c727-0ded-48e8-967c-e3cc1e35b8bc".to_string();
        let contenido = "¿Pregunta de prueba?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "tipo_invalido".to_string();
        let imagen_ref = None;

        let alternativas = HashMap::new();
        let puntaje = HashMap::new();

        let result = PreguntaEntity::new(
            id,
            contenido,
            etiqueta,
            tipo_de_pregunta,
            imagen_ref,
            alternativas,
            puntaje,
        );

        assert!(result.is_err());
    }

    #[test]
    fn test_parse_map_with_empty_map() {
        let empty_map: HashMap<String, String> = HashMap::new();
        let result = PreguntaEntity::parse_map(empty_map);

        assert!(result.is_ok());
        assert!(result.unwrap().is_none());
    }
}
