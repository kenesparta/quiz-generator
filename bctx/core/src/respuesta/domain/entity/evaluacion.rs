use crate::evaluacion::value_object::id::EvaluacionID;
use crate::respuesta::domain::entity::examen::Examen;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<Examen>,
}
