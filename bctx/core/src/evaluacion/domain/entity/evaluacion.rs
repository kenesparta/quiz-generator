use crate::examen::domain::entity::examen::ExamenList;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use quizz_common::domain::value_objects::fecha::FechaTiempoValueObject;

pub struct Evaluacion {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub fecha_tiempo_inicio: FechaTiempoValueObject,
    pub fecha_tiempo_fin: FechaTiempoValueObject,
    pub estado: EstadoGeneral,
    pub pregunta_id: String,
    pub examen_list: Option<ExamenList>,
}
