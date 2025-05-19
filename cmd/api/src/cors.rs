use actix_cors::Cors;

pub fn set_cors() -> Cors {
    Cors::default()
        .allowed_origin("http://127.0.0.1:5173/")
        .allowed_origin("http://127.0.0.1:5173")
        .allowed_origin("http://localhost:5173/")
        .allowed_origin("http://localhost:5173")
        .allowed_methods(vec!["GET", "POST", "PUT", "DELETE"])
        .allowed_headers(vec![
            http::header::AUTHORIZATION,
            http::header::ACCEPT,
            http::header::CONTENT_TYPE,
        ])
        .max_age(3600)
}
