use actix_cors::Cors;
use actix_web::http::header;

/// CORS restringido a los orígenes de la configuración (`cors.allowed_origins`).
pub fn set_cors(allowed_origins: &[String]) -> Cors {
    allowed_origins
        .iter()
        .fold(Cors::default(), |cors, origin| cors.allowed_origin(origin))
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
