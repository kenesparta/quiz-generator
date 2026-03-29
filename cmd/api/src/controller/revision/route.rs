use crate::controller::revision::listar_revisiones::ListarRevisionesController;
use crate::controller::revision::revisar_evaluacion_postulante::RevisarEvaluacionPostulanteController;
use actix_web::web;

pub fn revision(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/revisiones")
            .service(web::resource("").route(web::get().to(ListarRevisionesController::list)))
            .service(
                web::resource("/{respuesta_id}")
                    .route(web::post().to(RevisarEvaluacionPostulanteController::review)),
            ),
    );
}
