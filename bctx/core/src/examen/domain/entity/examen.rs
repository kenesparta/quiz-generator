use crate::examen::domain::error::examen::ExamenError;
use crate::examen::domain::value_object::id::ExamenID;

#[derive(Debug, Clone)]
pub struct Examen {
    id: ExamenID,
    titulo: String,
    descripcion: String,
    activo: bool,
}

impl Examen {
    pub fn new(
        id: String,
        titulo: String,
        descripcion: String,
        activo: bool,
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
        })
    }
}
