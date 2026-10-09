use crate::examen::domain::value_object::id::ExamenID;
use crate::respuesta::domain::entity::pregunta::Pregunta;

pub struct Examen {
    pub id: ExamenID,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub observaciones: String,
    pub preguntas: Vec<Pregunta>,
    pub observacion: String,
}

impl Examen {
    /// Suma de los puntos de las preguntas. Se calcula al leer en vez de guardarse: un total
    /// guardado al finalizar quedaba desactualizado si una respuesta llegaba en paralelo.
    pub fn puntos_obtenidos(&self) -> i64 {
        self.preguntas.iter().fold(0_i64, |total, pregunta| {
            total.saturating_add(pregunta.puntos)
        })
    }
}
