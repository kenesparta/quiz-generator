use crate::controller::admin::crypto::CifradoAdmin;
use crate::controller::admin::dto::RegistrarAdminDTO;
use crate::controller::admin::mongo::write::AdminMongo;
use crate::controller::error::ApiError;
use actix_web::{HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::admin::use_case::registrar_admin::{InputData, RegistrarAdmin};

pub struct AdminController;

impl AdminController {
    pub async fn create(
        id: web::Path<String>,
        body: web::Json<RegistrarAdminDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        RegistrarAdmin::new(Box::new(CifradoAdmin), Box::new(AdminMongo::new(db)))
            .ejecutar(InputData {
                id: id.into_inner(),
                nombre: dto.nombre,
                primer_apellido: dto.primer_apellido,
                segundo_apellido: dto.segundo_apellido,
                documento: dto.documento,
                password: dto.password,
            })
            .await?;
        Ok(HttpResponse::Created().finish())
    }
}
