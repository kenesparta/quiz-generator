use crate::examen::domain::error::examen::ExamenError;
use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;

#[derive(Debug, Clone)]
pub struct Examen {
    pub id: ExamenID,
    pub titulo: String,
    pub descripcion: String,
    pub activo: bool,
    pub puntaje_maximo: u32,
    pub preguntas: Option<ListaDePreguntas>,
}

impl Examen {
    pub fn new(
        id: String,
        titulo: String,
        descripcion: String,
        activo: bool,
        puntaje_maximo: u32,
        preguntas: Option<ListaDePreguntas>,
    ) -> Result<Self, ExamenError> {
        if titulo.trim().is_empty() {
            return Err(ExamenError::TituloInvalido);
        }

        if descripcion.trim().is_empty() {
            return Err(ExamenError::DescripcionInvalida);
        }

        let id = ExamenID::new(&id)?;
        Ok(Self {
            id,
            titulo,
            descripcion,
            activo,
            puntaje_maximo,
            preguntas,
        })
    }
}

pub struct ExamenList {
    pub examenes: Vec<Examen>,
}
