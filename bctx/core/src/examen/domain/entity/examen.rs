use quizz_common::domain::value_objects::fecha::FechaValueObject;
use crate::examen::domain::value_object::estado::Estado;
use crate::examen::domain::value_object::id::ExamenID;
use crate::examen::domain::value_object::version::Version;
use crate::pregunta::domain::entity::pregunta::{PreguntaEntity};
use crate::pregunta::domain::service::tipo_pregunta::TipoDePregunta;


pub struct Examen {
    pub id: ExamenID,
    pub nombre: String,
    pub descripcion: String,
    pub fecha: FechaValueObject,
    pub estado: Estado,
    pub version: Version,
    pub preguntas: Vec<TipoDePregunta>,
}
