use crate::controller::auth::middleware::Autorizacion;
use crate::controller::error::error_de_extraccion;
use crate::controller::examen::listar_examenes::ListarExamenesController;
use crate::controller::examen::registrar_examen::ExamenController;
use crate::controller::pregunta::agregar_pregunta::AgregarPreguntaController;
use actix_web::web;
use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;

/// Las preguntas pueden traer imágenes en base64 dentro del JSON; MongoDB limita cada
/// documento a 16 MB, así que una carga no puede acercarse a eso.
const LIMITE_JSON_PREGUNTAS_BYTES: usize = 8 * 1024 * 1024;

pub fn examen(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/examenes")
            .wrap(Autorizacion::para(Recurso::Examen))
            .service(web::resource("").route(web::get().to(ListarExamenesController::list)))
            .service(
                web::resource("/{id}")
                    .app_data(
                        web::JsonConfig::default()
                            .limit(LIMITE_JSON_PREGUNTAS_BYTES)
                            .error_handler(error_de_extraccion),
                    )
                    .route(web::post().to(ExamenController::create))
                    .route(web::put().to(AgregarPreguntaController::create)),
            ),
    );
}
