use actix_web::web;

pub fn examen(cfg: &mut web::ServiceConfig) {
    // cfg.service(
    //     web::scope("/examen").service(
    //         web::resource("/{id}")
    //             .route(web::post().to(ExamenControlller::create))
    //             .route(web::put().to(AgregarPreguntaController::create)),
    //     ),
    // );
}
