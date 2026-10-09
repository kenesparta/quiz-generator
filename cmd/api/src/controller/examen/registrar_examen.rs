use crate::controller::error::ApiError;
use crate::controller::examen::dto::RegistrarExamenDTO;
use crate::mongo::examen::write::ExamenMongo;
use actix_web::{HttpResponse, web};
use quizz_core::examen::use_case::crear_examen::{CrearExamen, InputData};

pub struct ExamenController;

impl ExamenController {
    pub async fn create(
        id: web::Path<String>,
        body: web::Json<RegistrarExamenDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        CrearExamen::new(ExamenMongo::new(db))
            .ejecutar(InputData {
                id: id.into_inner(),
                titulo: dto.titulo,
                descripcion: dto.descripcion,
                instrucciones: dto.instrucciones,
            })
            .await?;
        Ok(HttpResponse::Created().finish())
    }
}
