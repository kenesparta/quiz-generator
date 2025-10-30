use actix_web::{HttpRequest, HttpResponse, web};

pub struct ListarPostulanteController {}

impl ListarPostulanteController {
    pub async fn get(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
