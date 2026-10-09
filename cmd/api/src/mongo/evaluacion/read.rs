use crate::mongo::evaluacion::write::EvaluacionMongo;
use crate::mongo::repositorio::MongoRepository;
use futures::TryStreamExt;
use mongodb::bson::doc;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::LecturaNoFinalizada;
use quizz_core::evaluacion::provider::repositorio::RepositorioEvaluacionListar;
use quizz_core::evaluacion::use_case::listar_evaluaciones::OutputData;
use serde::Deserialize;
use tracing::error;

/// Una evaluación en el listado: sin los exámenes (una publicada guarda copias completas, con
/// imágenes), solo cuántos tiene.
#[derive(Deserialize)]
struct FilaListaEvaluacion {
    #[serde(rename = "_id")]
    id: String,
    nombre: String,
    descripcion: String,
    estado: String,
    esta_activo: String,
    cantidad_examenes: i64,
}

impl RepositorioEvaluacionListar for EvaluacionMongo {
    async fn listar_evaluaciones(&self) -> Result<Vec<OutputData>, EvaluacionError> {
        let error_lectura = |e: mongodb::error::Error| {
            error!("mongo, listar evaluaciones: {e}");
            EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada)
        };
        let filas: Vec<FilaListaEvaluacion> = self
            .get_collection()
            .clone_with_type::<FilaListaEvaluacion>()
            .find(doc! {})
            .projection(doc! {
                "nombre": 1,
                "descripcion": 1,
                "estado": 1,
                "esta_activo": 1,
                "cantidad_examenes": { "$size": { "$ifNull": ["$examenes", []] } },
            })
            .await
            .map_err(error_lectura)?
            .try_collect()
            .await
            .map_err(error_lectura)?;

        Ok(filas
            .into_iter()
            .map(|fila| OutputData {
                id: fila.id,
                nombre: fila.nombre,
                descripcion: fila.descripcion,
                estado: fila.estado,
                esta_activo: fila.esta_activo,
                cantidad_examenes: usize::try_from(fila.cantidad_examenes).unwrap_or(0),
            })
            .collect())
    }
}
