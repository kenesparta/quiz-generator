use crate::respuesta::domain::entity::examen::Examen;

pub struct Evaluacion {
    pub nombre: String,
    pub descripcion: String,
    pub examenes: Vec<Examen>,
}
