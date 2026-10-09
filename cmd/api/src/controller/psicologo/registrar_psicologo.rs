use crate::controller::cifrado::Bcrypt;
use crate::controller::error::ApiError;
use crate::controller::psicologo::dto::RegistrarPsicologoDTO;
use crate::controller::psicologo::mongo::write::PsicologoMongo;
use actix_web::{HttpResponse, web};
use quizz_core::psicologo::use_case::registrar_psicologo::{InputData, RegistrarPsicologo};

pub struct PsicologoController;

impl PsicologoController {
    pub async fn create(
        id: web::Path<String>,
        body: web::Json<RegistrarPsicologoDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        RegistrarPsicologo::new(Bcrypt::default(), PsicologoMongo::new(db))
            .ejecutar(InputData {
                id: id.into_inner(),
                nombre: dto.nombre,
                primer_apellido: dto.primer_apellido,
                segundo_apellido: dto.segundo_apellido,
                documento: dto.documento,
                especialidad: dto.especialidad,
                colegiatura: dto.colegiatura,
                password: dto.password,
            })
            .await?;
        Ok(HttpResponse::Created().finish())
    }
}
