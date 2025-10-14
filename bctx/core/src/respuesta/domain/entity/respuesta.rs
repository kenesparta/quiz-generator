use crate::evaluacion::value_object::id::EvaluacionID;
use crate::examen::domain::value_object::id::ExamenID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::respuesta::domain::entity::evaluacion::Evaluacion;
use crate::respuesta::domain::value_object::id::RespuestaID;

pub struct Respuesta {
    pub id: RespuestaID,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion: Evaluacion,
    pub postulante: PostulanteID,
    // pub estado: RespuestaEstado,
}

enum RespuestaEstado {
    Creado,
    EnProceso,
    Finalizado,
}

pub struct RespuestaEvaluacion {
    pub id: RespuestaID,
    pub postulante_id: PreguntaID,
    pub evaluacion_id: String,
    pub examen_id: String,
    pub pregunta_id: String,
    pub respuestas: Vec<String>,
}
