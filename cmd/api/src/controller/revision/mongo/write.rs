use crate::controller::mongo_repository::MongoRepository;
use crate::controller::revision::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use mongodb::bson::{Document, doc};
use quizz_common::domain::value_objects::zona_horaria::formatear_rfc3339;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Revision};
use quizz_core::respuesta::domain::entity::revision::RevisionRealizada;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::RepositorioRealizarRevision;
use tracing::error;

pub struct RevisionEvaluacionMongo {
    client: web::Data<mongodb::Database>,
}

impl RevisionEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RevisionEvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

fn error_bd(contexto: &'static str) -> impl FnOnce(mongodb::error::Error) -> RespuestaError {
    move |e| {
        error!("mongo, {contexto}: {e}");
        RespuestaError::DatabaseError
    }
}

impl RepositorioRealizarRevision for RevisionEvaluacionMongo {
    async fn realizar_revision(&self, revision: &RevisionRealizada) -> Result<(), RespuestaError> {
        let respuesta_id = revision.respuesta_id.to_string();
        let examen_ids: Vec<&str> = revision
            .examenes
            .iter()
            .map(|examen| examen.examen_id.as_str())
            .collect();

        // Todas las condiciones van en el filtro y todo se escribe en una sola operación: la
        // hoja finalizada (ya no cambia) y todos los exámenes revisados presentes en ella.
        let mut filtro = doc! {
            "_id": &respuesta_id,
            "evaluacion._id": &revision.evaluacion_id,
            "estado": Estado::Finalizado.to_string(),
        };
        if !examen_ids.is_empty() {
            filtro.insert("evaluacion.examenes._id", doc! { "$all": &examen_ids });
        }

        let mut cambios = doc! {
            "revision": Revision::Finalizada.to_string(),
            "resultado": &revision.resultado,
            "revisado_por": &revision.revisado_por,
            "fecha_revision": formatear_rfc3339(&revision.fecha),
        };
        let mut array_filters: Vec<Document> = Vec::new();
        for (indice, examen) in revision.examenes.iter().enumerate() {
            let identificador = format!("examen{indice}");
            cambios.insert(
                format!("evaluacion.examenes.$[{identificador}].observacion"),
                &examen.observacion,
            );
            array_filters.push(doc! { format!("{identificador}._id"): &examen.examen_id });
        }

        let resultado = self
            .get_collection()
            .update_one(filtro, doc! { "$set": cambios })
            .array_filters(array_filters)
            .await
            .map_err(error_bd("guardar la revision"))?;
        if resultado.matched_count > 0 {
            return Ok(());
        }

        // No se escribió: averiguar por qué para responder con el error correcto.
        let hoja = self
            .get_collection()
            .find_one(doc! { "_id": &respuesta_id, "evaluacion._id": &revision.evaluacion_id })
            .projection(doc! { "estado": 1 })
            .await
            .map_err(error_bd("leer la hoja a revisar"))?
            .ok_or(RespuestaError::RespuestaNoEncontrada)?;
        if hoja.get_str("estado").ok() != Some(&Estado::Finalizado.to_string()) {
            return Err(RespuestaError::EvaluacionNoFinalizada);
        }
        Err(RespuestaError::ExamenNotFound)
    }
}
