use crate::controller::examen::dto::RegistrarExamenDTO;
use crate::controller::examen::mongo::write::ExamenMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::examen::use_case::crear_examen::{CrearExamen, InputData};
use tracing::log::error;

pub struct ExamenControlller;

impl ExamenControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarExamenDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let examen_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("no se esta enviando el id del examen");
            }
        };

        let registrar_examen = CrearExamen::new(Box::new(ExamenMongo::new(pool)));
        let dto = body.into_inner();
        let input = InputData {
            id: examen_id,
            titulo: dto.titulo,
            descripcion: dto.descripcion,
            instrucciones: dto.instrucciones,
        };

        match registrar_examen.ejecutar(input).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(e) => {
                error!("error al registrar el examen: {}", e);
                HttpResponse::InternalServerError().json("error al registrar el examen")
            }
        }
    }
}
