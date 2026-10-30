use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links};
use crate::controller::revision::dto::{
    RevisionDetalleDTO, RevisionEvaluacionDTO, RevisionExamenDTO, RevisionPreguntaDTO,
};
use crate::mongo::psicologo::write::PsicologoMongo;
use crate::mongo::respuesta::RespuestaMongo;
use actix_web::{HttpResponse, web};
use quizz_core::respuesta::use_case::obtener_revision::{InputData, ObtenerRevisionPorId};

pub struct ObtenerRevisionController;

impl ObtenerRevisionController {
    /// Una hoja finalizada con su calificación. El psicólogo que se muestra es la firma de
    /// quien la calificó (guardada en la hoja), no quien la consulta.
    pub async fn get(
        respuesta_id: web::Path<String>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let output =
            ObtenerRevisionPorId::new(RespuestaMongo::new(db.clone()), PsicologoMongo::new(db))
                .ejecutar(InputData {
                    revision_id: respuesta_id.into_inner(),
                })
                .await?;

        let mut links = Links::new();
        links.insert(
            "self".into(),
            Link::get(format!("/revisiones/{}", output.id)),
        );
        links.insert(
            "postulante".into(),
            Link::get(format!("/postulantes?id={}", output.postulante_id)),
        );

        Ok(HttpResponse::Ok().json(RevisionDetalleDTO {
            id: output.id,
            postulante_id: output.postulante_id,
            resultado: output.resultado,
            revision: output.revision,
            fecha_tiempo_inicio: output.fecha_tiempo_inicio,
            fecha_tiempo_fin: output.fecha_tiempo_fin,
            fecha_revision: output.fecha_revision,
            psicologo: output.firma.map(Into::into),
            evaluacion: RevisionEvaluacionDTO {
                id: output.evaluacion.id,
                nombre: output.evaluacion.nombre,
                descripcion: output.evaluacion.descripcion,
                examenes: output
                    .evaluacion
                    .examenes
                    .into_iter()
                    .map(|ex| RevisionExamenDTO {
                        id: ex.id,
                        titulo: ex.titulo,
                        descripcion: ex.descripcion,
                        instrucciones: ex.instrucciones,
                        preguntas: ex
                            .preguntas
                            .into_iter()
                            .map(|p| RevisionPreguntaDTO {
                                id: p.id,
                                contenido: p.contenido,
                                tipo_de_pregunta: p.tipo_de_pregunta,
                                etiqueta: p.etiqueta,
                                imagen_ref: Some(p.imagen_ref).filter(|i| !i.is_empty()),
                                alternativas: p.alternativas.into_iter().collect(),
                                respuestas: p.respuestas,
                                puntos: p.puntos,
                            })
                            .collect(),
                        puntos_obtenidos: ex.puntos_obtenidos,
                        observacion: ex.observacion,
                    })
                    .collect(),
            },
            links,
        }))
    }
}
