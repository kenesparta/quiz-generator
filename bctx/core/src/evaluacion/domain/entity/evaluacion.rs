use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::value_object::examen_id::ExamenIDs;
use crate::evaluacion::value_object::id::EvaluacionID;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use std::str::FromStr;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub estado: EstadoGeneral,
    pub examen_list: ExamenIDs,
}

impl Evaluacion {
    pub fn new(
        id: String,
        nombre: String,
        descripcion: String,
        estado: String,
        examen_list: Vec<String>,
    ) -> Result<Self, EvaluacionError> {
        if nombre.trim().is_empty() {
            return Err(EvaluacionError::NombreNoValido);
        }

        if descripcion.trim().is_empty() {
            return Err(EvaluacionError::DescripcionNoValida);
        }

        let id = EvaluacionID::new(&id)?;
        let examen_list = ExamenIDs::new(examen_list);
        let estado = EstadoGeneral::from_str(&estado)?;

        Ok(Self {
            id,
            nombre,
            descripcion,
            estado,
            examen_list,
        })
    }
}
