use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;

pub struct Pregunta {
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: HashMap<String, String>,
    pub puntaje: HashMap<String, u32>,
    pub respuesta: HashMap<String, u32>,
}
