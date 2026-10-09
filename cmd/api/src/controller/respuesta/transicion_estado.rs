use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links};
use crate::controller::respuesta::dto::{AccionTransicion, TransicionEstadoDTO};
use crate::controller::respuesta::mongo::write::EstadoRespuestaMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_common::domain::value_objects::zona_horaria::ahora_lima;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::domain::entity::respuesta::Estado;
use quizz_core::respuesta::use_case::empezar_examen::{
    EmpezarExamen, InputData as EmpezarInputData,
};
use quizz_core::respuesta::use_case::finalizar_evaluacion::{
    FinalizarEvaluacion, InputData as FinalizarInputData,
};
use serde_json::json;

pub struct TransicionEstadoController;

impl TransicionEstadoController {
    /// `PATCH /respuestas/{id}/estado` con `{"accion": "empezar" | "finalizar"}`. Solo el
    /// postulante dueño de la hoja (el del token) puede cambiar su estado.
    pub async fn transicionar(
        req: HttpRequest,
        respuesta_id: web::Path<String>,
        body: web::Json<TransicionEstadoDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let respuesta_id = respuesta_id.into_inner();
        let repositorio = Box::new(EstadoRespuestaMongo::new(db));
        let mut links = Links::new();
        links.insert(
            "self".into(),
            Link::get(format!("/respuestas/{respuesta_id}")),
        );

        let (estado, mensaje) = match body.into_inner().accion {
            AccionTransicion::Empezar => {
                EmpezarExamen::new(repositorio)
                    .ejecutar(EmpezarInputData {
                        id: respuesta_id.clone(),
                        postulante_id: claims.sub,
                        ahora: ahora_lima(),
                    })
                    .await?;
                links.insert(
                    "finalizar".into(),
                    Link::patch(format!("/respuestas/{respuesta_id}/estado")),
                );
                (Estado::EnProceso, "Examen iniciado correctamente")
            }
            AccionTransicion::Finalizar => {
                FinalizarEvaluacion::new(repositorio)
                    .ejecutar(FinalizarInputData {
                        id: respuesta_id,
                        postulante_id: claims.sub,
                        ahora: ahora_lima(),
                    })
                    .await?;
                (Estado::Finalizado, "Examen finalizado correctamente")
            }
        };

        Ok(HttpResponse::Ok().json(json!({
            "estado": estado.to_string(),
            "mensaje": mensaje,
            "_links": links
        })))
    }
}
