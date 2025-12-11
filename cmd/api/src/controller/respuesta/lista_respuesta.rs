use crate::controller::respuesta::mongo::read::RespuestaRevisionMongo;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaRevisionDTO;
use actix_web::{HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::respuesta_revision::RespuestaRevision;

pub struct ListaRespuestaController;

impl ListaRespuestaController {
    pub async fn list_respuestas_revision(pool: web::Data<mongodb::Client>) -> HttpResponse {
        let revision = RespuestaRevision::new(Box::new(RespuestaRevisionMongo::new(pool)));

        match revision.ejecutar(()).await {
            Ok(r) => HttpResponse::Ok().json(
                r.into_iter()
                    .map(|rev| RespuestaRevisionDTO {
                        // revision_id u respuesta_id son lo mismo
                        revision_id: rev.revision_id,
                        nombre_evaluacion: rev.nombre_evaluacion,
                        descripcion_evaluacion: rev.descripcion_evaluacion,
                        estado_revision: rev.estado_revision,
                        postulante_id: rev.postulante_id,
                    })
                    .collect::<Vec<RespuestaRevisionDTO>>(),
            ),

            Err(_) => HttpResponse::InternalServerError().finish(),
        }
    }
}

// todo
// Del lado del postulante:
// - Lista de evaluacion por postulante
// - Endpoint para iniciar evaluacion
// Del lado del admin:
// - Obtener el puntaje de cada postulante por examen y pregunta
