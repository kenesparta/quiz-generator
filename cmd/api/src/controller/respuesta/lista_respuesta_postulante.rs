use crate::controller::respuesta::mongo::read::ListaRespuestaPostulanteMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::lista_respuesta_postulante::{
    InputData, ListaRespuestaPostulante,
};
use serde::Serialize;
use serde_json::json;

#[derive(Serialize)]
pub struct ListaRespuestaPostulanteDTO {
    pub respuesta_id: String,
    pub nombre_evaluacion: String,
    pub descripcion_evaluacion: String,
    pub estado: String,
}

pub struct ListaRespuestaPostulanteController;

impl ListaRespuestaPostulanteController {
    pub async fn list(req: HttpRequest, pool: web::Data<mongodb::Client>) -> HttpResponse {
        let postulante_id = match req.match_info().get("postulante_id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("Se debe enviar el ID del postulante");
            }
        };

        let lista_respuesta =
            ListaRespuestaPostulante::new(Box::new(ListaRespuestaPostulanteMongo::new(pool)));
        let input = InputData { postulante_id };

        match lista_respuesta.ejecutar(input).await {
            Ok(respuestas) => HttpResponse::Ok().json(
                respuestas
                    .into_iter()
                    .map(|r| ListaRespuestaPostulanteDTO {
                        respuesta_id: r.respuesta_id,
                        nombre_evaluacion: r.nombre_evaluacion,
                        descripcion_evaluacion: r.descripcion_evaluacion,
                        estado: r.estado,
                    })
                    .collect::<Vec<ListaRespuestaPostulanteDTO>>(),
            ),
            Err(e) => HttpResponse::InternalServerError().json(json!({"error": e.to_string()})),
        }
    }
}