use crate::controller::respuesta::contestar_pregunta::ContestarPreguntaController;
use crate::controller::respuesta::listar_respuestas::ListarRespuestasController;
use crate::controller::respuesta::obtener_respuesta::ObtenerRespuestaController;
use crate::controller::respuesta::transicion_estado::TransicionEstadoController;
use actix_web::web;

pub fn respuesta(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/respuestas")
            .service(
                web::resource("")
                    .route(web::get().to(ListarRespuestasController::list)),
            )
            .service(web::resource("/{id}").route(web::get().to(ObtenerRespuestaController::get)))
            .service(
                web::resource("/{id}/estado")
                    .route(web::patch().to(TransicionEstadoController::transicionar)),
            )
            .service(
                web::resource("/{id}/examenes/{examen_id}/preguntas/{pregunta_id}/contestaciones")
                    .route(web::post().to(ContestarPreguntaController::contestar)),
            ),
    );
}
