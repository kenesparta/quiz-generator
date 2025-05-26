use crate::controller::pregunta::dto::{PreguntaInputDto, PreguntaRawDataDto};
use crate::controller::pregunta::mongo::write::PreguntaPorExamenMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::pregunta::use_case::agregar_preguntas::{
    AgregarPreguntas, InputData, PreguntaRawData,
};

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
                return HttpResponse::BadRequest().json("no se esta enviando el id del examen");
            }
        };

        let agregar_preguntas = AgregarPreguntas::new(Box::new(PreguntaPorExamenMongo::new(pool)));

        let dto = body.into_inner();
        let preguntas: Vec<PreguntaRawData> = dto
            .preguntas
            .into_iter()
            .map(|p| match p {
                PreguntaRawDataDto::Alternativas {
                    id,
                    contenido,
                    imagen_ref,
                    alternativa_correcta,
                    alternativas,
                } => PreguntaRawData::Alternativas {
                    id,
                    contenido,
                    imagen_ref,
                    alternativa_correcta,
                    alternativas,
                },
                PreguntaRawDataDto::Libre {
                    id,
                    contenido,
                    imagen_ref,
                } => PreguntaRawData::Libre {
                    id,
                    contenido,
                    imagen_ref,
                },
                PreguntaRawDataDto::SolaRespuesta {
                    id,
                    contenido,
                    imagen_ref,
                    respuesta_correcta,
                } => PreguntaRawData::SolaRespuesta {
                    id,
                    contenido,
                    imagen_ref,
                    respuesta_correcta,
                },
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
                _ => HttpResponse::InternalServerError().json("Error inesperado"),
            },
        }
    }
}
