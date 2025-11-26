use crate::controller::respuesta::mongo::read::RespuestaRevisionMongo;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaRevisionDTO;
use actix_web::{HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::use_case::respuesta_revision::{OutputData, RespuestaRevision};

pub struct ListaRespuestaController;

impl ListaRespuestaController {
    pub async fn list_respuestas_revision(pool: web::Data<mongodb::Client>) -> HttpResponse {
        let revision = RespuestaRevision::new(Box::new(RespuestaRevisionMongo::new(pool)));

        match revision.ejecutar(()).await {
            Ok(r) => HttpResponse::Ok().json(
                r.into_iter()
                    .map(|rev| RespuestaRevisionDTO {
                        nombre_evaluacion: rev.nombre_evaluacion,
                        descripcion_evaluacion: rev.descripcion_evaluacion,
                        postulante_id: rev.postulante_id,
                    })
                    .collect::<Vec<RespuestaRevisionDTO>>(),
            ),

            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}
