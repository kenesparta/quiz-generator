use crate::controller::respuesta::asignar_evaluacion_postulante::AsignarEvaluacionPostulanteController;
use crate::controller::respuesta::empezar_examen::EmpezarExamenController;
use crate::controller::respuesta::lista_respuesta::ListaRespuestaController;
use crate::controller::respuesta::lista_respuesta_postulante::ListaRespuestaPostulanteController;
use crate::controller::respuesta::responder_evaluacion::ResponderEvaluacionController;
use crate::controller::respuesta::respuesta_por_postulante::RespuestaPorPostulanteController;
use actix_web::web;

pub fn respuesta(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/respuesta")
            .service(
                web::resource("")
                    .route(web::post().to(AsignarEvaluacionPostulanteController::create)),
            )
            .service(
                web::resource("/revision")
                    .route(web::get().to(ListaRespuestaController::list_respuestas_revision)),
            )
            .service(
                web::resource("/postulante/{postulante_id}")
                    .route(web::get().to(ListaRespuestaPostulanteController::list)),
            )
            .service(
                web::resource("/{id}/postulante/{postulante_id}")
                    .route(web::get().to(RespuestaPorPostulanteController::read)),
            )
            .service(
                web::resource("/{id}")
                    .route(web::patch().to(ResponderEvaluacionController::response)),
            )
            .service(
                web::resource("/{id}/finalizar")
                    .route(web::patch().to(ResponderEvaluacionController::finalizar)),
            )
            .service(
                web::resource("/{id}/empezar")
                    .route(web::patch().to(EmpezarExamenController::empezar)),
            ),
    );
}
