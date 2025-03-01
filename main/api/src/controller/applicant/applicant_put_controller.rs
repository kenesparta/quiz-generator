use actix_web::{web, HttpRequest, HttpResponse, Responder};
use quizz_core::applicant::application::applicant_sign_upper::ApplicantSignUpper;

#[derive(serde::Deserialize, Clone)]
pub struct ApplicantRequestDTO {
    pub name: String,
    pub first_lastname: String,
    pub second_lastname: String,
}

pub struct ApplicantPutController {
    sign_upper: ApplicantSignUpper,
}

impl ApplicantPutController {
    fn new(id: String, dto: ApplicantRequestDTO) -> Self {
        let sign_upper = ApplicantSignUpper::new(id);
        ApplicantPutController { sign_upper }
    }

    // This method will be called by Actix
    pub async fn update(req: HttpRequest, body: web::Json<ApplicantRequestDTO>) -> HttpResponse {
        let id = req.match_info().get("id").unwrap_or_default().to_string();
        let controller = Self::new(id, body.clone());

        // Call your business logic
        // let result = controller.sign_upper.update(body.into_inner());

        HttpResponse::Ok().finish()
    }
}
