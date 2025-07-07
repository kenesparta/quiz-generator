use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::value_object::id::EvaluacionID;
use crate::examen::domain::service::lista_examenes::ListaDeExamenes;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use std::str::FromStr;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub estado: EstadoGeneral,
    pub examenes: ListaDeExamenes,
}

impl Evaluacion {
    pub fn new(
        id: String,
        nombre: String,
        descripcion: String,
        estado: String,
    ) -> Result<Self, EvaluacionError> {
        if nombre.trim().is_empty() {
            return Err(EvaluacionError::NombreNoValido);
        }

        if descripcion.trim().is_empty() {
            return Err(EvaluacionError::DescripcionNoValida);
        }

        let id = EvaluacionID::new(&id)?;
        let estado = EstadoGeneral::from_str(&estado)?;

        Ok(Self {
            id,
            nombre,
            descripcion,
            estado,
            examenes: ListaDeExamenes::new(Vec::new()),
        })
    }
}
