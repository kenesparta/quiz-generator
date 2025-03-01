use actix_web::{web, HttpRequest, HttpResponse};
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

pub struct ApplicantPutController {
    sign_upper: ApplicantSignUpper,
}

impl ApplicantPutController {
    fn new(id: String, req: ApplicantRequestDTO) -> Self {
        let applicant_sign_upper_dto = ApplicantSignUpperDTO {
            name: req.name,
            first_lastname: req.first_lastname,
            second_lastname: req.second_lastname,
            document: req.document,
        };
        let sign_upper = ApplicantSignUpper::new(id, applicant_sign_upper_dto);
        ApplicantPutController { sign_upper }
    }

    pub async fn update(req: HttpRequest, body: web::Json<ApplicantRequestDTO>) -> HttpResponse {
        let id = req.match_info().get("id").unwrap_or_default().to_string();
        let controller = Self::new(id, body.clone());
        HttpResponse::Ok().finish()
    }
}
