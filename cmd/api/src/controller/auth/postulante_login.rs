use crate::controller::auth::dto::PostulanteLoginRequestDTO;
use actix_web::{HttpRequest, HttpResponse, web};

pub struct PostulanteLoginController;

impl PostulanteLoginController {
    pub async fn login(
        _req: HttpRequest,
        body: web::Json<PostulanteLoginRequestDTO>,
        _pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let _dto = body.into_inner();
        HttpResponse::Ok().finish()
    }
}
