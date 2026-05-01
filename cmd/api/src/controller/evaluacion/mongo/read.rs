use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::mongo_repository::MongoRepository;
use async_trait::async_trait;
use log::error;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::LecturaNoFinalizada;
use quizz_core::evaluacion::provider::repositorio::RepositorioEvaluacionListar;
use quizz_core::evaluacion::use_case::listar_evaluaciones::OutputData;

#[async_trait]
impl RepositorioEvaluacionListar<EvaluacionError> for EvaluacionMongo {
    async fn listar_evaluaciones(&self) -> Result<Vec<OutputData>, EvaluacionError> {
        let mut cursor = self.get_collection().find(doc! {}).await.map_err(|e| {
            error!("Database error while listing evaluaciones: {}", e);
            EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada)
        })?;

        let mut evaluaciones = Vec::new();

        while cursor.advance().await.map_err(|e| {
            error!("Error advancing cursor while listing evaluaciones: {}", e);
            EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada)
        })? {
            let documento = cursor.deserialize_current().map_err(|e| {
                error!("Error deserializing evaluacion document: {}", e);
                EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada)
            })?;

            let id = documento
                .get_str("_id")
                .map_err(|_| EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada))?
                .to_string();

            let nombre = documento
                .get_str("nombre")
                .map_err(|_| EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada))?
                .to_string();

            let descripcion = documento
                .get_str("descripcion")
                .map_err(|_| EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada))?
                .to_string();

            let estado = documento
                .get_str("estado")
                .map_err(|_| EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada))?
                .to_string();

            let esta_activo = documento
                .get_str("esta_activo")
                .map_err(|_| EvaluacionError::EvaluacionRepositorioError(LecturaNoFinalizada))?
                .to_string();

            let cantidad_examenes = match documento.get("examenes") {
                Some(bson::Bson::Array(arr)) => arr.len(),
                _ => 0,
            };

            evaluaciones.push(OutputData {
                id,
                nombre,
                descripcion,
                estado,
                esta_activo,
                cantidad_examenes,
            });
        }

        Ok(evaluaciones)
    }
}
