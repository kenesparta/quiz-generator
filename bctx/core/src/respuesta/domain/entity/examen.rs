use crate::respuesta::domain::entity::pregunta::Pregunta;

pub struct Examen {
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub preguntas: Vec<Pregunta>,
}
