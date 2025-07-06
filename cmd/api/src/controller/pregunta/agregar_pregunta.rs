use crate::controller::pregunta::dto::PreguntaInputDto;
use crate::controller::pregunta::mongo::write::PreguntaPorExamenMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::pregunta::use_case::agregar_preguntas::{
    AgregarPreguntasParaExamen, InputData, PreguntaEntityInput,
};
use std::collections::HashMap;
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

        let agregar_preguntas =
            AgregarPreguntasParaExamen::new(Box::new(PreguntaPorExamenMongo::new(pool)));
        let dto = body.into_inner();
        let preguntas = dto
            .preguntas
            .into_iter()
            .map(|dto| PreguntaEntityInput {
                contenido: dto.contenido,
                etiqueta: dto.etiqueta,
                tipo_de_pregunta: dto.tipo_de_pregunta,
                imagen_ref: dto.imagen_ref,
                alternativas: dto.alternativas.unwrap_or_else(|| HashMap::new()),
                puntaje: dto.puntaje.unwrap_or_else(|| HashMap::new()),
            })
            .collect();

        let input = InputData {
            examen_id,
            preguntas,
        };

        match agregar_preguntas.ejecutar(input).await {
            Ok(_) => HttpResponse::Ok().finish(),
            Err(err) => match err {
                PreguntaError::RespuestaNoExiste => {
                    HttpResponse::BadRequest().json("La pregunta debe tener una respuesta")
                }
                PreguntaError::RespuestaIncorrecta => {
                    HttpResponse::BadRequest().json("La respuesta proporcionada es incorrecta")
                }
                PreguntaError::AlternativasNoExisten => {
                    HttpResponse::BadRequest().json("La pregunta debe tener alternativas")
                }
                PreguntaError::AlternativasVacias => {
                    HttpResponse::BadRequest().json("Las alternativas no pueden estar vacías")
                }
                PreguntaError::PuntajeNoExiste => {
                    HttpResponse::BadRequest().json("La pregunta debe tener un puntaje definido")
                }
                PreguntaError::PuntajeVacio => {
                    HttpResponse::BadRequest().json("El puntaje no puede estar vacío")
                }
                PreguntaError::PuntajeNoCoincideConAlternativa => HttpResponse::BadRequest()
                    .json("El puntaje debe coincidir con las alternativas disponibles"),
                PreguntaError::PreguntaErrorExamenID(id_error) => HttpResponse::BadRequest()
                    .json(format!("Error en el ID del examen: {}", id_error)),
                PreguntaError::PreguntaAlternativaError(alt_error) => HttpResponse::BadRequest()
                    .json(format!("Error en las alternativas: {}", alt_error)),
                PreguntaError::PreguntaEtiquetaError(etiqueta_error) => HttpResponse::BadRequest()
                    .json(format!("Error en la etiqueta: {}", etiqueta_error)),
                PreguntaError::PreguntaTipoPreguntaError(tipo_error) => HttpResponse::BadRequest()
                    .json(format!("Error en el tipo de pregunta: {}", tipo_error)),
                PreguntaError::PreguntaRepositorioError(repo_error) => {
                    log::error!("Repository error: {}", repo_error);
                    HttpResponse::InternalServerError().json("Error interno del servidor")
                }
                PreguntaError::DebeTenerUnaSolaRespuesta => {
                    HttpResponse::BadRequest().json("La pregunta debe tener una sola respuesta")
                }
                PreguntaError::AlternativaNoAjustada => HttpResponse::BadRequest().json(""),
                PreguntaError::PuntajeNoAjustado => HttpResponse::BadRequest().json(""),
            },
        }
    }
}
