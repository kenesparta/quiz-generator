use actix_web::{HttpRequest, HttpResponse, web};
use sqlx::PgPool;
use quizz_core::postulante::use_case::registrar_postulante::RegistrarPostulantePasswordTemporal;
use crate::controller::postulante::crypto::CifradoPorDefecto;
use crate::controller::postulante::datasbase::PostulantePostgres;
use crate::controller::postulante::dto::RegistrarPostulanteDTO;

pub struct PostulantePutController;

impl PostulantePutController {
    pub async fn put(req: HttpRequest, body: web::Json<RegistrarPostulanteDTO>, pool: web::Data<PgPool>) -> HttpResponse {
        let id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("Missing applicant ID in the request path");
            }
        };
        let postulante_pool = PostulantePostgres::new(pool);
        let registrar_postulante = RegistrarPostulantePasswordTemporal::new(
            Box::new(CifradoPorDefecto),
            Box::new(postulante_pool),
        );
        //
        // let mut sign_upper = match ApplicantSignUpper::new() {
        //     Ok(service) => service,
        //     Err(e) => {
        //         return HttpResponse::InternalServerError()
        //             .json(format!("Failed to create sign upper service: {}", e));
        //     }
        // };
        //
        // let body = body.into_inner();
        // let applicant_sign_upper_dto = ApplicantSignUpperDTO {
        //     name: body.name,
        //     first_lastname: body.first_lastname,
        //     second_lastname: body.second_lastname,
        //     document: body.document,
        // };
        //
        // match sign_upper.insert_applicant(id, applicant_sign_upper_dto) {
        //     Ok(_) => HttpResponse::Ok().json(""),
        //     Err(e) => HttpResponse::BadRequest().json(format!("Failed to update applicant: {}", e)),
        // }
        HttpResponse::Ok().json("")
    }
}
