use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::value_object::id::EvaluacionID;
use crate::examen::domain::entity::examen::ExamenList;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub examen_list: Option<ExamenList>,
}

impl Evaluacion {
    pub fn new(
        id: String,
        nombre: String,
        descripcion: String,
        examen_list: Option<ExamenList>,
    ) -> Result<Self, EvaluacionError> {
        if nombre.trim().is_empty() {
            return Err(EvaluacionError::NombreNoValido);
        }

        if descripcion.trim().is_empty() {
            return Err(EvaluacionError::DescripcionNoValida);
        }

        let id = EvaluacionID::new(&id)?;
        Ok(Self {
            id,
            nombre,
            descripcion,
            examen_list,
        })
    }
}
