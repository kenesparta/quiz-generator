use crate::controller::evaluacion::dto::RegistrarEvaluacionDTO;
use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::evaluacion::use_case::crear_evaluacion::{CrearEvaluacion, InputData};

pub struct EvaluacionControlller;

impl EvaluacionControlller {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarEvaluacionDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let evaluacion_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("POST /evaluacion - id no proporcionado");
                return HttpResponse::BadRequest()
                    .json("no se esta enviando el id de la evaluacion");
            }
        };

        info!("POST /evaluacion/{}", evaluacion_id);

        let dto = body.into_inner();
        let input = InputData {
            id: evaluacion_id.clone(),
            titulo: dto.titulo,
            descripcion: dto.descripcion,
        };

        let registrar_evaluacion = CrearEvaluacion::new(Box::new(EvaluacionMongo::new(pool)));
        match registrar_evaluacion.ejecutar(input).await {
            Ok(_) => {
                info!("POST /evaluacion/{} - creada exitosamente", evaluacion_id);
                HttpResponse::Created().finish()
            }
            Err(e) => {
                error!(
                    "POST /evaluacion/{} - error al registrar: {}",
                    evaluacion_id, e
                );
                HttpResponse::InternalServerError().json("error al registrar la evaluacion")
            }
        }
    }
}
