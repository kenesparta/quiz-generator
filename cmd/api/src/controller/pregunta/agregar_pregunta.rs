use crate::controller::error::ApiError;
use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::pregunta::dto::PreguntaInputDto;
use actix_web::{HttpResponse, web};
use quizz_core::pregunta::use_case::agregar_preguntas::{
    AgregarPreguntasParaExamen, InputData, PreguntaEntityInput,
};

pub struct AgregarPreguntaController;

impl AgregarPreguntaController {
    pub async fn create(
        examen_id: web::Path<String>,
        body: web::Json<PreguntaInputDto>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let preguntas = body
            .into_inner()
            .preguntas
            .into_iter()
            .map(|dto| PreguntaEntityInput {
                contenido: dto.contenido,
                etiqueta: dto.etiqueta,
                tipo_de_pregunta: dto.tipo_de_pregunta,
                imagen_ref: dto.imagen_ref,
                alternativas: dto.alternativas.unwrap_or_default(),
                puntaje: dto.puntaje.unwrap_or_default(),
            })
            .collect();

        AgregarPreguntasParaExamen::new(ExamenMongo::new(db))
            .ejecutar(InputData {
                examen_id: examen_id.into_inner(),
                preguntas,
            })
            .await?;
        Ok(HttpResponse::Ok().finish())
    }
}
