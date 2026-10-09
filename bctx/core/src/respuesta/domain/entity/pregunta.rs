use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use crate::respuesta::domain::entity::respuesta::Estado;
use std::collections::HashMap;

pub type Puntaje = HashMap<String, u32>;

/// Lo necesario para corregir una contestación: el estado de la hoja de respuestas y la
/// pregunta tal como quedó en su copia de la evaluación (incluida la clave de corrección).
pub struct PreguntaACorregir {
    pub estado: Estado,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: HashMap<String, String>,
    pub puntaje: Puntaje,
}

pub struct Pregunta {
    pub id: PreguntaID,
    pub contenido: String,
    pub observaciones: String,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,

    // puntaje se refiere al puntaje dado por pregunta
    pub puntaje: Puntaje,
    pub respuestas: Option<Vec<String>>,
    pub puntos: i64,
}

pub fn corregir_respuesta(respuesta: &[String], puntaje: Puntaje) -> u32 {
    respuesta.iter().filter_map(|key| puntaje.get(key)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_corregir_respuesta_suma_correcta() {
        let respuesta = vec!["D".to_string(), "B".to_string()];
        let puntaje = HashMap::from([
            ("A".to_string(), 0),
            ("B".to_string(), 4),
            ("C".to_string(), 2),
            ("D".to_string(), 1),
            ("E".to_string(), 3),
        ]);

        let resultado = corregir_respuesta(&respuesta, puntaje);
        assert_eq!(resultado, 5);
    }

    #[test]
    fn test_corregir_respuesta_unica_incorrecta() {
        let respuesta: Vec<String> = vec!["B".to_string()];
        let puntaje = HashMap::from([("A".to_string(), 1)]);

        let resultado = corregir_respuesta(&respuesta, puntaje);
        assert_eq!(resultado, 0);
    }

    #[test]
    fn test_corregir_respuesta_unica_correcta() {
        let respuesta: Vec<String> = vec!["A".to_string()];
        let puntaje = HashMap::from([("A".to_string(), 1)]);

        let resultado = corregir_respuesta(&respuesta, puntaje);
        assert_eq!(resultado, 1);
    }
}
