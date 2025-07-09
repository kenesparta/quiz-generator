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
    pub instrucciones: String,
    pub estado: EstadoGeneral,
    pub preguntas: ListaDePreguntas,
}

impl Examen {
    pub fn new(
        id: String,
        titulo: String,
        descripcion: String,
        instrucciones: String,
    ) -> Result<Self, ExamenError> {
        if titulo.trim().is_empty() {
            return Err(ExamenError::TituloInvalido);
        }

        if descripcion.trim().is_empty() {
            return Err(ExamenError::DescripcionInvalida);
        }

        let estado = EstadoGeneral::Activo;
        let id = ExamenID::new(&id)?;

        Ok(Self {
            id,
            titulo,
            descripcion,
            instrucciones,
            estado,
            preguntas: ListaDePreguntas::new(Vec::new()),
        })
    }
}
