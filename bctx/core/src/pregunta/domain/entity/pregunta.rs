use crate::pregunta::domain::entity::tipo_pregunta_strategy::get_strategy;
use crate::pregunta::domain::error::alternativa::AlternativaError;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

pub trait PreguntaProps: Clone + PartialEq + Debug {
    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError>;
    fn obtener_puntaje(&self) -> Result<HashMap<Etiqueta, u32>, PreguntaError>;
}

#[derive(Debug, Clone)]
pub struct PreguntaEntity {
    pub id: PreguntaID,
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: Option<HashMap<Alternativa, String>>,
    pub puntos: Option<HashMap<Alternativa, u32>>,
}

impl PreguntaEntity {
    pub fn new(
        id: String,
        contenido: String,
        etiqueta: String,
        tipo_de_pregunta: String,
        imagen_ref: Option<String>,
        alternativas: HashMap<String, String>,
        puntos: HashMap<String, u32>,
    ) -> Result<Self, PreguntaError> {
        let id = PreguntaID::new(&id)?;
        let etiqueta = Etiqueta::from_str(&etiqueta)?;
        let tipo_de_pregunta = TipoPregunta::from_str(&tipo_de_pregunta)?;
        let alternativas = Self::parse_map(alternativas)?;
        let puntos = Self::parse_map(puntos)?;

        let strategy = get_strategy(&tipo_de_pregunta);
        let alternativas = strategy.ajustar_alternativas(alternativas)?;
        let puntos = strategy.ajustar_puntos(puntos)?;

        strategy.verificar_consistencia(&alternativas, &puntos)?;

        Ok(Self {
            id,
            contenido,
            etiqueta,
            tipo_de_pregunta,
            alternativas,
            puntos,
            imagen_ref,
        })
    }

    // pub fn obtener_puntaje(&self, respuesta: String) -> Result<u32, PreguntaError> {
    //     match &self.alternativas {
    //         None => Ok(0),
    //         Some(alternativas) => {
    //             let respuesta = respuesta.parse::<Alternativa>()?;
    //             if !alternativas.contains_key(&respuesta) {
    //                 return Err(PreguntaError::RespuestaIncorrecta);
    //             }
    //             match &self.puntos {
    //                 None => Ok(0),
    //                 Some(puntos) => {
    //                     if !puntos.contains_key(&respuesta) {
    //                         return Err(PreguntaError::RespuestaNoExiste);
    //                     }
    //                     Ok(0)
    //                 }
    //             }
    //         }
    //     }
    // }

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
