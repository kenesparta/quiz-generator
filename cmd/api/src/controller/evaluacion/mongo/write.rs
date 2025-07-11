use std::str::FromStr;
use crate::controller::evaluacion::mongo::constantes::EVALUACION_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_core::evaluacion::domain::entity::evaluacion::Evaluacion;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::{
    EvaluacionNoExiste, PersistenciaNoFinalizada,
};
use quizz_core::evaluacion::provider::repositorio::{
    RepositorioEvaluacionEscritura, RepositorioLeerEvaluacion, RepositorioPublicarEvaluacion,
};
use quizz_core::evaluacion::value_object::examen_id::ExamenIDs;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use tracing::log::error;
use quizz_common::domain::value_objects::estado::EstadoGeneral;

pub struct EvaluacionMongo {
    client: web::Data<mongodb::Client>,
}

impl EvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        EvaluacionMongo { client }
    }
}

impl MongoRepository for EvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        EVALUACION_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioEvaluacionEscritura<EvaluacionError> for EvaluacionMongo {
    async fn guardar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), EvaluacionError> {
        let documento = doc! {
            "_id": evaluacion.id.to_string(),
            "nombre": evaluacion.nombre,
            "descripcion": evaluacion.descripcion,
            "estado": evaluacion.esta_activo.to_string(),
        };

        match self.get_collection().insert_one(documento, None).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!("Error al guardar evaluacion: {e}");
                Err(EvaluacionError::EvaluacionRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn agregar_examen(
        &self,
        evaluacion_id: EvaluacionID,
        examen_ids: ExamenIDs,
    ) -> Result<(), EvaluacionError> {
        let evaluacion_exists = self
            .get_collection()
            .find_one(
                doc! {
                    "_id": evaluacion_id.to_string()
                },
                None,
            )
            .await
            .map_err(|e| {
                error!("Error checking if exam exists: {}", e);
                EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
            })?;

        if evaluacion_exists.is_none() {
            return Err(EvaluacionError::EvaluacionRepositorioError(
                EvaluacionNoExiste,
            ));
        }

        let examen_ids_strings: Vec<String> = examen_ids
            .examen_ids
            .iter()
            .map(|id| id.uuid().to_string())
            .collect();

        let update = doc! {
            "$addToSet": {
                "examenes": {
                    "$each": examen_ids_strings
                }
            }
        };

        match self
            .get_collection()
            .update_one(
                doc! {
                    "_id": evaluacion_id.to_string()
                },
                update,
                None,
            )
            .await
        {
            Ok(result) => {
                if result.matched_count == 0 {
                    error!("No evaluation found with ID: {}", evaluacion_id.to_string());
                    return Err(EvaluacionError::EvaluacionRepositorioError(
                        EvaluacionNoExiste,
                    ));
                }
                Ok(())
            }
            Err(e) => {
                error!("Error updating evaluation with exam IDs: {}", e);
                Err(EvaluacionError::EvaluacionRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }
}

#[async_trait]
impl RepositorioLeerEvaluacion<EvaluacionError> for EvaluacionMongo {
    async fn obtener_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
    ) -> Result<Evaluacion, EvaluacionError> {
        let documento = self
            .get_collection()
            .find_one(
                doc! {
                    "_id": evaluacion_id.to_string()
                },
                None,
            )
            .await
            .map_err(|e| {
                error!("Error al buscar evaluacion: {}", e);
                EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
            })?;

        match documento {
            Some(doc) => {
                let id = doc.get_str("_id")
                    .map_err(|e| {
                        error!("Error al obtener ID de evaluacion: {}", e);
                        EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                    })?;

                let nombre = doc.get_str("nombre")
                    .map_err(|e| {
                        error!("Error al obtener nombre de evaluacion: {}", e);
                        EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                    })?;

                let descripcion = doc.get_str("descripcion")
                    .map_err(|e| {
                        error!("Error al obtener descripcion de evaluacion: {}", e);
                        EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                    })?;

                let estado_str = doc.get_str("estado")
                    .map_err(|e| {
                        error!("Error al obtener estado de evaluacion: {}", e);
                        EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                    })?;

                // Parse the estado back to EstadoGeneral
                let esta_activo = EstadoGeneral::from_str(estado_str)?;

                // Get examenes array if it exists
                let examenes_ids = doc.get_array("examenes")
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|item| item.as_str())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();

                // Create the Evaluacion entity
                let mut evaluacion = Evaluacion::new(
                    id.to_string(),
                    nombre.to_string(),
                    descripcion.to_string(),
                )?;

                // Set the estado_activo to match what was stored
                evaluacion.esta_activo = esta_activo;

                

                Ok(evaluacion)
            }
            None => Err(EvaluacionError::EvaluacionRepositorioError(EvaluacionNoExiste)),
        }
    }
}

#[async_trait]
impl RepositorioPublicarEvaluacion<EvaluacionError> for EvaluacionMongo {
    async fn publicar_evaluacion(&self, evaluacion: Evaluacion) -> Result<(), EvaluacionError> {
        todo!()
    }
}
