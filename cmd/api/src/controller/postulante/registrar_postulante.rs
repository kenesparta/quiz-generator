use actix_web::{HttpRequest, HttpResponse, web};
use quizz_core::applicant::application::applicant_sign_upper::{
    ApplicantSignUpper, ApplicantSignUpperDTO,
};

#[derive(serde::Deserialize, Clone)]
pub struct ApplicantRequestDTO {
    pub name: String,
    pub first_lastname: String,
    pub second_lastname: String,
    pub document: String,
}

pub struct ApplicantPutController {}

impl ApplicantPutController {
    pub async fn update(req: HttpRequest, body: web::Json<ApplicantRequestDTO>) -> HttpResponse {
        let id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("Missing applicant ID in the request path");
            }
        };

        let mut sign_upper = match ApplicantSignUpper::new() {
            Ok(service) => service,
            Err(e) => {
                return HttpResponse::InternalServerError()
                    .json(format!("Failed to create sign upper service: {}", e));
            }
        };

        let body = body.into_inner();
        let applicant_sign_upper_dto = ApplicantSignUpperDTO {
            name: body.name,
            first_lastname: body.first_lastname,
            second_lastname: body.second_lastname,
            document: body.document,
        };

        match sign_upper.insert_applicant(id, applicant_sign_upper_dto) {
            Ok(_) => HttpResponse::Ok().json(""),
            Err(e) => HttpResponse::BadRequest().json(format!("Failed to update applicant: {}", e)),
        }
    }
}
