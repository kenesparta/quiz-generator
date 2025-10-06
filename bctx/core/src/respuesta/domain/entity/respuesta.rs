use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::value_object::id::RespuestaID;

pub struct Respuesta {
    pub id: RespuestaID,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion: Evaluacion,
    pub postulante: PostulanteID,
}
