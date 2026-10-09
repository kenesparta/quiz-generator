use crate::controller::auth::middleware::Autorizacion;
use crate::controller::revision::listar_revisiones::ListarRevisionesController;
use crate::controller::revision::obtener_revision::ObtenerRevisionController;
use crate::controller::revision::revisar_evaluacion_postulante::RevisarEvaluacionPostulanteController;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

pub fn revision(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/revisiones")
            .wrap(Autorizacion::para(Recurso::Revision))
            .service(web::resource("").route(web::get().to(ListarRevisionesController::list)))
            .service(
                web::resource("/{respuesta_id}")
                    .route(web::get().to(ObtenerRevisionController::get))
                    .route(web::post().to(RevisarEvaluacionPostulanteController::review))
                    .route(web::patch().to(RevisarEvaluacionPostulanteController::review)),
            ),
    );
}
