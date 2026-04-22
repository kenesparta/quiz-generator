use crate::controller::respuesta::dto::{
    CrearRespuestaDTO, RespuestaCreatedDTO, build_respuesta_links,
};
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::asignar_postulante::{
    AsignarEvaluacionAPostulante, InputData,
};
use serde_json::json;

pub struct AsignarEvaluacionPostulanteController;

impl AsignarEvaluacionPostulanteController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<CrearRespuestaDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let evaluacion_id = match req.match_info().get("evaluacion_id") {
            Some(id) => id.to_string(),
            None => {
                warn!("POST /evaluaciones/.../respuestas - evaluacion_id no proporcionado");
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la evaluacion"}));
            }
        };

        let dto = body.into_inner();

        info!(
            "POST /evaluaciones/{}/respuestas - postulante={}",
            evaluacion_id, dto.postulante_id
        );

        let asociar =
            AsignarEvaluacionAPostulante::new(Box::new(RespuestaEvaluacionMongo::new(pool)));

        let input = InputData {
            evaluacion_id: evaluacion_id.clone(),
            postulante_id: dto.postulante_id.clone(),
        };

        match asociar.ejecutar(input).await {
            Ok(()) => {
                info!(
                    "POST /evaluaciones/{}/respuestas - asignacion exitosa",
                    evaluacion_id
                );
                let links = build_respuesta_links(
                    "",
                    &dto.postulante_id,
                    "Creado",
                    &Rol::Psicologo.to_string(),
                );
                HttpResponse::Created().json(RespuestaCreatedDTO {
                    id: String::new(),
                    estado: "Creado".to_string(),
                    links,
                })
            }
            Err(err) => {
                error!(
                    "POST /evaluaciones/{}/respuestas - error: {}",
                    evaluacion_id, err
                );
                HttpResponse::InternalServerError().json(json!({"error": err.to_string()}))
            }
        }
    }
}
