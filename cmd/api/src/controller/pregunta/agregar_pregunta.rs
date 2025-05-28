use crate::controller::pregunta::dto::PreguntaInputDto;
use crate::controller::pregunta::mongo::write::PreguntaPorExamenMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::pregunta::use_case::agregar_preguntas::{
    AgregarPreguntas, InputData, PreguntaEntityInput,
};
use tracing::log;

pub struct AgregarPreguntaController;

impl AgregarPreguntaController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<PreguntaInputDto>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let examen_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("se debe enviar el ID del examen");
            }
        };

        let agregar_preguntas = AgregarPreguntas::new(Box::new(PreguntaPorExamenMongo::new(pool)));
        let dto = body.into_inner();
        let preguntas = dto
            .preguntas
            .into_iter()
            .map(|dto| PreguntaEntityInput {
                id: dto.id,
                contenido: dto.contenido,
                etiqueta: dto.etiqueta,
                tipo_de_pregunta: dto.tipo_de_pregunta,
                imagen_ref: dto.imagen_ref,
                alternativas: dto.alternativas,
                puntaje: dto.puntaje,
            })
            .collect();

        let input = InputData {
            examen_id,
            preguntas,
        };

        match agregar_preguntas.ejecutar(input).await {
            Ok(_) => HttpResponse::Created().json(""),
            Err(err) => match err {
                // PreguntaError::PreguntaIdError(id_err) => {
                //     HttpResponse::BadRequest().json(format!("Error de ID: {}", id_err))
                // },
                // PreguntaError::ExamenIdError(examen_id_err) => {
                //     HttpResponse::BadRequest().json(format!("Error de ID de examen: {}", examen_id_err))
                // },
                // PreguntaError::PreguntaRepositorioError(_repo_err) => {
                //     // log::error!("Error de repositorio: {}", repo_err);
                //     HttpResponse::InternalServerError().json("Error al guardar las preguntas")
                // },
                _ => {
                    log::error!("Error de repositorio: {}", err);
                    HttpResponse::InternalServerError().json("Error inesperado")
                }
            },
        }
    }
}
