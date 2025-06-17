use crate::examen::domain::error::examen::ExamenError;
use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Examen {
    pub id: ExamenID,
    pub titulo: String,
    pub descripcion: String,
    pub estado: EstadoGeneral,
    pub puntaje_maximo: u32,
    pub preguntas: Option<ListaDePreguntas>,
}

impl Examen {
    pub fn new(
        id: String,
        titulo: String,
        descripcion: String,
        estado: String,
        puntaje_maximo: u32,
        preguntas: Option<ListaDePreguntas>,
    ) -> Result<Self, ExamenError> {
        if titulo.trim().is_empty() {
            return Err(ExamenError::TituloInvalido);
        }

        if descripcion.trim().is_empty() {
            return Err(ExamenError::DescripcionInvalida);
        }

        if puntaje_maximo == 0 {
            return Err(ExamenError::PuntajeIgualQueCero);
        }

        let estado = EstadoGeneral::from_str(&estado)?;
        let id = ExamenID::new(&id)?;

        Ok(Self {
            id,
            titulo,
            descripcion,
            estado,
            puntaje_maximo,
            preguntas,
        })
    }
}
