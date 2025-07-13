use crate::controller::evaluacion::dto::RegistrarEvaluacionDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::crear_evaluacion::{CrearEvaluacion, InputData};
use tracing::log::error;

pub struct EvaluacionControlller;

impl EvaluacionControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarEvaluacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let dto = body.into_inner();
        let input = InputData {
            id: match req.match_info().get("id") {
                Some(id) => id.to_string(),
                None => {
                    return HttpResponse::BadRequest()
                        .json("no se esta enviando el id de la evaluacion");
                }
            },
            titulo: dto.titulo,
            descripcion: dto.descripcion,
        };

        let registrar_evaluacion = CrearEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match registrar_evaluacion.ejecutar(input).await {
            Ok(_) => HttpResponse::Created().finish(),
            Err(e) => {
                error!("error al registrar la evaluacion: {e}");
                HttpResponse::InternalServerError().json("error al registrar la evaluacion")
            }
        }
    }
}
