use crate::controller::revision::revisar_evaluacion_postulante::RevisarEvaluacionPostulanteController;
use actix_web::web;

pub fn revision(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/revision").service(
            web::resource("/{id}")
                .route(web::patch().to(RevisarEvaluacionPostulanteController::review)),
        ),
    );
}
