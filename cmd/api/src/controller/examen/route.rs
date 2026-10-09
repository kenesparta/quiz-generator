use crate::controller::auth::middleware::Autorizacion;
use crate::controller::examen::listar_examenes::ListarExamenesController;
use crate::controller::examen::registrar_examen::ExamenControlller;
use crate::controller::pregunta::agregar_pregunta::AgregarPreguntaController;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

pub fn examen(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/examenes")
            .wrap(Autorizacion::para(Recurso::Examen))
            .service(web::resource("").route(web::get().to(ListarExamenesController::list)))
            .service(
                web::resource("/{id}")
                    .route(web::post().to(ExamenControlller::create))
                    .route(web::put().to(AgregarPreguntaController::create)),
            ),
    );
}
