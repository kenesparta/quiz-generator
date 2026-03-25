use crate::controller::respuesta::mongo::write::RepositorioEmpezarExamenMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::empezar_examen::{EmpezarExamen, InputData};
use serde_json::json;

pub struct EmpezarExamenController;

impl EmpezarExamenController {
    pub async fn empezar(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("PATCH /respuesta/empezar - id no proporcionado");
                return HttpResponse::BadRequest().json("Se debe enviar el ID de la respuesta");
            }
        };

        info!("PATCH /respuesta/{}/empezar", respuesta_id);

        let empezar_examen = EmpezarExamen::new(Box::new(RepositorioEmpezarExamenMongo::new(pool)));
        let input = InputData {
            id: respuesta_id.clone(),
        };

        match empezar_examen.ejecutar(input).await {
            Ok(_) => {
                info!(
                    "PATCH /respuesta/{}/empezar - examen iniciado",
                    respuesta_id
                );
                HttpResponse::Ok().json(json!({"message": "Examen iniciado correctamente"}))
            }
            Err(e) => {
                error!("PATCH /respuesta/{}/empezar - error: {}", respuesta_id, e);
                HttpResponse::InternalServerError().json(json!({"error": e.to_string()}))
            }
        }
    }
}
