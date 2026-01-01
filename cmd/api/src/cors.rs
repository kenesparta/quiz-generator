use actix_cors::Cors;
use actix_web::http::header;

pub fn set_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://127.0.0.1:3000/")
        .allowed_origin("http://127.0.0.1:3000")
        .allowed_origin("http://localhost:3000/")
        .allowed_origin("http://localhost:3000")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "PATCH"])
        .allowed_headers(vec![
            header::AUTHORIZATION,
            header::ACCEPT,
            header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
