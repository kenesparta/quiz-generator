use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links};
use crate::controller::psicologo::mongo::read::PsicologoReadMongo;
use crate::controller::revision::dto::{
    RevisionDetalleDTO, RevisionEvaluacionDTO, RevisionExamenDTO, RevisionPreguntaDTO,
    RevisionPsicologoDTO,
};
use crate::controller::revision::mongo::read::RevisionReadMongo;
use actix_web::{HttpResponse, web};
use log::warn;
use quizz_common::use_case::CasoDeUso;
use quizz_core::psicologo::provider::repositorio::RepositorioPsicologoLectura;
use quizz_core::respuesta::use_case::obtener_revision::{InputData, ObtenerRevisionPorId};

pub struct ObtenerRevisionController;

impl ObtenerRevisionController {
    /// Una hoja finalizada con su calificación. El psicólogo que se muestra es el que la
    /// calificó (guardado en la hoja), no quien la consulta.
    pub async fn get(
        respuesta_id: web::Path<String>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let output = ObtenerRevisionPorId::new(Box::new(RevisionReadMongo::new(db.clone())))
            .ejecutar(InputData {
                revision_id: respuesta_id.into_inner(),
            })
            .await?;

        // Hojas calificadas antes de registrar al revisor, o por un admin: sin psicólogo.
        let psicologo = match &output.revisado_por {
            Some(revisor) => PsicologoReadMongo::new(db)
                .obtener_psicologo_por_id(revisor.clone())
                .await
                .map_err(|e| warn!("revisor {revisor} no es un psicologo legible: {e:?}"))
                .ok()
                .map(|info| RevisionPsicologoDTO {
                    nombre_completo: format!(
                        "{} {} {}",
                        info.nombre, info.primer_apellido, info.segundo_apellido
                    ),
                    colegiatura: info.colegiatura,
                }),
            None => None,
        };

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
            psicologo,
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
