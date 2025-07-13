use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::value_object::id::RespuestaID;
use quizz_common::domain::value_objects::fecha::{FechaTiempoValueObject};
use crate::respuesta::domain::entity::correccion::Correccion;

pub struct Respuesta {
    pub id: RespuestaID,
    pub fecha_tiempo_inicio: FechaTiempoValueObject,
    pub fecha_tiempo_fin: FechaTiempoValueObject,
    pub correccion: Correccion,
    pub evaluacion: Evaluacion,
}
