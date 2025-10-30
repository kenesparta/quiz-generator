use crate::controller::revision::dto::{
    RevisarEvaluacionPostulanteFinalizeDTO, RevisarEvaluacionPostulanteReadDTO,
    RevisarEvaluacionPostulanteReviewDTO,
};
use actix_web::{HttpRequest, HttpResponse, web};

pub struct RevisarEvaluacionPostulanteController {}

impl RevisarEvaluacionPostulanteController {
    pub async fn read(
        req: HttpRequest,
        body: web::Json<RevisarEvaluacionPostulanteReadDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn review(
        req: HttpRequest,
        body: web::Json<RevisarEvaluacionPostulanteReviewDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }

    pub async fn finalize(
        req: HttpRequest,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Ok().finish()
    }
}
