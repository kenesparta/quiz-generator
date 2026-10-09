use crate::controller::error::ApiError;
use crate::controller::respuesta::dto::{
    CrearRespuestaDTO, RespuestaCreatedDTO, build_respuesta_links,
};
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::http::header;
use actix_web::{HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::domain::entity::respuesta::Estado;
use quizz_core::respuesta::use_case::asignar_postulante::{
    AsignarEvaluacionAPostulante, InputData,
};

pub struct AsignarEvaluacionPostulanteController;

impl AsignarEvaluacionPostulanteController {
    /// `POST /evaluaciones/{evaluacion_id}/respuestas`: crea la hoja de respuestas y devuelve su
    /// id (también en `Location`). Asignar dos veces la misma evaluación responde 409.
    pub async fn create(
        evaluacion_id: web::Path<String>,
        body: web::Json<CrearRespuestaDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let creada = AsignarEvaluacionAPostulante::new(Box::new(RespuestaEvaluacionMongo::new(db)))
            .ejecutar(InputData {
                evaluacion_id: evaluacion_id.into_inner(),
                postulante_id: body.into_inner().postulante_id,
            })
            .await?;

        let id = creada.id.to_string();
        Ok(HttpResponse::Created()
            .insert_header((header::LOCATION, format!("/respuestas/{id}")))
            .json(RespuestaCreatedDTO {
                links: build_respuesta_links(&id, Estado::Creado, Rol::Psicologo),
                estado: Estado::Creado.to_string(),
                id,
            }))
    }
}
