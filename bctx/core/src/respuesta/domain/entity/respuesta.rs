use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::value_object::id::RespuestaID;
use quizz_common::domain::value_objects::fecha::FechaTiempoValueObject;

pub struct Respuesta {
    pub id: RespuestaID,
    // pub fecha_tiempo_inicio: FechaTiempoValueObject,
    // pub fecha_tiempo_fin: FechaTiempoValueObject,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion: Evaluacion,
    pub postulante: PostulanteID,
}
