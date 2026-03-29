use crate::controller::auth::jwt::Claims;
use crate::controller::hateoas::Link;
use crate::controller::respuesta::dto::TransicionEstadoDTO;
use crate::controller::respuesta::mongo::write::{
    RepositorioEmpezarExamenMongo, RespositorioFinalizarEvaluacionMongo,
};
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::empezar_examen::{
    EmpezarExamen, InputData as EmpezarInputData,
};
use quizz_core::respuesta::use_case::finalizar_evaluacion::{
    FinalizarEvaluacion, InputData as FinalizarInputData,
};
use serde_json::json;

pub struct TransicionEstadoController;

impl TransicionEstadoController {
    pub async fn transicionar(
        req: HttpRequest,
        body: web::Json<TransicionEstadoDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let respuesta_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest()
                    .json(json!({"error": "Se debe enviar el ID de la respuesta"}));
            }
        };

        let claims = match req.extensions().get::<Claims>().cloned() {
            Some(c) => c,
            None => {
                return HttpResponse::Unauthorized().json(json!({"error": "Token no encontrado"}));
            }
        };

        let rol = claims.rol.as_deref().unwrap_or("");
        let dto = body.into_inner();

        info!(
            "PATCH /respuestas/{}/estado - accion={}",
            respuesta_id, dto.accion
        );

        match dto.accion.as_str() {
            "empezar" => Self::empezar(pool, &respuesta_id, rol).await,
            "finalizar" => Self::finalizar(pool, &respuesta_id, rol).await,
            _ => {
                warn!(
                    "PATCH /respuestas/{}/estado - accion no valida: {}",
                    respuesta_id, dto.accion
                );
                HttpResponse::BadRequest()
                    .json(json!({"error": "Accion no valida. Use 'empezar' o 'finalizar'"}))
            }
        }
    }

    async fn empezar(
        pool: web::Data<mongodb::Client>,
        respuesta_id: &str,
        _rol: &str,
    ) -> HttpResponse {
        let empezar_examen = EmpezarExamen::new(Box::new(RepositorioEmpezarExamenMongo::new(pool)));
        let input = EmpezarInputData {
            id: respuesta_id.to_string(),
        };

        match empezar_examen.ejecutar(input).await {
            Ok(_) => {
                info!(
                    "PATCH /respuestas/{}/estado - examen iniciado",
                    respuesta_id
                );
                let mut links = crate::controller::hateoas::Links::new();
                links.insert(
                    "self".into(),
                    Link::get(format!("/respuestas/{}", respuesta_id)),
                );
                links.insert(
                    "finalizar".into(),
                    Link::patch(format!("/respuestas/{}/estado", respuesta_id)),
                );

                HttpResponse::Ok().json(json!({
                    "estado": "EnProceso",
                    "mensaje": "Examen iniciado correctamente",
                    "_links": links
                }))
            }
            Err(e) => {
                error!(
                    "PATCH /respuestas/{}/estado empezar - error: {}",
                    respuesta_id, e
                );
                HttpResponse::Conflict()
                    .json(json!({"error": "No se puede iniciar el examen en el estado actual"}))
            }
        }
    }

    async fn finalizar(
        pool: web::Data<mongodb::Client>,
        respuesta_id: &str,
        _rol: &str,
    ) -> HttpResponse {
        let input = FinalizarInputData {
            id: respuesta_id.to_string(),
        };
        let finalizar =
            FinalizarEvaluacion::new(Box::new(RespositorioFinalizarEvaluacionMongo::new(pool)));

        match finalizar.ejecutar(input).await {
            Ok(_) => {
                info!("PATCH /respuestas/{}/estado - finalizado", respuesta_id);
                let mut links = crate::controller::hateoas::Links::new();
                links.insert(
                    "self".into(),
                    Link::get(format!("/respuestas/{}", respuesta_id)),
                );

                HttpResponse::Ok().json(json!({
                    "estado": "Finalizado",
                    "mensaje": "Examen finalizado correctamente",
                    "_links": links
                }))
            }
            Err(e) => {
                error!(
                    "PATCH /respuestas/{}/estado finalizar - error: {}",
                    respuesta_id, e
                );
                HttpResponse::Conflict()
                    .json(json!({"error": "No se puede finalizar el examen en el estado actual"}))
            }
        }
    }
}
