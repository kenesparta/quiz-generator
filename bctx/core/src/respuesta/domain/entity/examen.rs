use crate::examen::domain::value_object::id::ExamenID;
use crate::respuesta::domain::entity::pregunta::Pregunta;

pub struct Examen {
    pub id: ExamenID,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub observaciones: String,
    pub preguntas: Vec<Pregunta>,
    pub puntos_obtenidos: i64,
    pub observacion: String,
}
