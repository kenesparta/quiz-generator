use actix_web::{HttpRequest, HttpResponse, web};

pub struct ResponderQuestionarioController;

impl ResponderQuestionarioController {
    pub async fn read(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        HttpResponse::Ok().json("Respuesta enviada correctamente")
    }
}
