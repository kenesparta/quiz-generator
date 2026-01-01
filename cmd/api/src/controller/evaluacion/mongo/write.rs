use crate::controller::evaluacion::mongo::constantes::EVALUACION_COLLECTION_NAME;
use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use quizz_core::evaluacion::domain::entity::evaluacion::Evaluacion;
use quizz_core::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_core::evaluacion::domain::error::evaluacion::RepositorioError::{
    EvaluacionNoExiste, PersistenciaNoFinalizada,
};
use quizz_core::evaluacion::domain::value_object::evaluacion_estado::EvaluacionEstado;
use quizz_core::evaluacion::provider::repositorio::{
    RepositorioEvaluacionEscritura, RepositorioLeerEvaluacion, RepositorioPublicarEvaluacion,
};
use quizz_core::evaluacion::value_object::examen_id::ExamenIDs;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::examen::domain::service::lista_examenes::ListaDeExamenes;
use quizz_core::examen::provider::repositorio::RepositorioExamenLectura;
use std::str::FromStr;
use tracing::log::error;

pub struct EvaluacionMongo {
    client: web::Data<mongodb::Client>,
    repositorio_examen: ExamenMongo,
}

impl EvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        let repositorio_examen = ExamenMongo::new(client.clone());
        EvaluacionMongo {
            client,
            repositorio_examen,
        }
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
            "esta_activo": evaluacion.esta_activo.to_string(),
            "estado": evaluacion.estado.to_string(),
        };

        match self.get_collection().insert_one(documento).await {
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
            .find_one(doc! {
                    "_id": evaluacion_id.to_string()
                })
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
            .find_one(doc! {
                    "_id": evaluacion_id.to_string()
                })
            .await
            .map_err(|e| {
                error!("Error al buscar evaluacion: {}", e);
                EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
            })?;

        match documento {
            Some(doc) => {
                let id = doc.get_str("_id").map_err(|e| {
                    error!("Error al obtener ID de evaluacion: {}", e);
                    EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                })?;

                let nombre = doc.get_str("nombre").map_err(|e| {
                    error!("Error al obtener nombre de evaluacion: {}", e);
                    EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                })?;

                let descripcion = doc.get_str("descripcion").map_err(|e| {
                    error!("Error al obtener descripcion de evaluacion: {}", e);
                    EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                })?;

                let estado_str = doc.get_str("estado").map_err(|e| {
                    error!("Error al obtener estado de evaluacion: {}", e);
                    EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                })?;

                let esta_activo_str = doc.get_str("esta_activo").map_err(|e| {
                    error!("Error al obtener si la evaluacion esta activa: {}", e);
                    EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
                })?;

                let estado = EvaluacionEstado::from_str(estado_str)?;
                let esta_activo = EstadoGeneral::from_str(esta_activo_str)?;

                let examenes_ids = doc
                    .get_array("examenes")
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|item| item.as_str())
                            .map(|s| s.to_string())
                            .collect::<Vec<String>>()
                    })
                    .unwrap_or_default();

                let examenes = if !examenes_ids.is_empty() {
                    let mut examenes_vec = Vec::new();

                    for examen_id in examenes_ids {
                        let examen = self
                            .repositorio_examen
                            .obtener_examen(examen_id.as_str())
                            .await
                            .map_err(|e| {
                                error!("Error al obtener examen {}: {}", examen_id, e);
                                EvaluacionError::EvaluacionRepositorioError(
                                    PersistenciaNoFinalizada,
                                )
                            })?;
                        examenes_vec.push(examen);
                    }
                    ListaDeExamenes::new(examenes_vec)
                } else {
                    ListaDeExamenes::new(Vec::new())
                };

                let mut evaluacion =
                    Evaluacion::new(id.to_string(), nombre.to_string(), descripcion.to_string())?;
                evaluacion.esta_activo = esta_activo;
                evaluacion.estado = estado;
                evaluacion.examenes = examenes;

                Ok(evaluacion)
            }
            None => Err(EvaluacionError::EvaluacionRepositorioError(
                EvaluacionNoExiste,
            )),
        }
    }
}

#[async_trait]
impl RepositorioPublicarEvaluacion<EvaluacionError> for EvaluacionMongo {
    async fn publicar_evaluacion(&self, mut evaluacion: Evaluacion) -> Result<(), EvaluacionError> {
        let evaluation_exists = self
            .get_collection()
            .find_one(doc! {
                    "_id": evaluacion.id.to_string()
                })
            .await
            .map_err(|e| {
                error!("Error checking if evaluation exists: {}", e);
                EvaluacionError::EvaluacionRepositorioError(PersistenciaNoFinalizada)
            })?;

        if evaluation_exists.is_none() {
            return Err(EvaluacionError::EvaluacionRepositorioError(
                EvaluacionNoExiste,
            ));
        }

        let examenes_docs: Vec<mongodb::bson::Document> = evaluacion
            .examenes
            .examenes()
            .iter()
            .map(|examen| {
                let preguntas_docs: Vec<mongodb::bson::Document> = examen
                    .preguntas
                    .preguntas()
                    .iter()
                    .map(|pregunta| {
                        let mut pregunta_doc = doc! {
                            "_id": pregunta.id.to_string(),
                            "contenido": &pregunta.contenido,
                            "etiqueta": pregunta.etiqueta.to_string(),
                            "tipo_de_pregunta": pregunta.tipo_de_pregunta.to_string(),
                        };

                        if let Some(ref imagen) = pregunta.imagen_ref {
                            pregunta_doc.insert("imagen_ref", imagen);
                        }

                        let alternativas_doc: mongodb::bson::Document = pregunta
                            .alternativas
                            .iter()
                            .map(|(key, value)| {
                                (key.clone(), mongodb::bson::Bson::String(value.clone()))
                            })
                            .collect();
                        pregunta_doc.insert("alternativas", alternativas_doc);

                        let puntaje_doc: mongodb::bson::Document = pregunta
                            .puntaje
                            .iter()
                            .map(|(key, value)| {
                                (key.clone(), mongodb::bson::Bson::Int32(*value as i32))
                            })
                            .collect();
                        pregunta_doc.insert("puntaje", puntaje_doc);

                        pregunta_doc
                    })
                    .collect();

                doc! {
                    "_id": examen.id.to_string(),
                    "titulo": &examen.titulo,
                    "descripcion": &examen.descripcion,
                    "instrucciones": &examen.instrucciones,
                    "estado": examen.estado.to_string(),
                    "preguntas": preguntas_docs,
                }
            })
            .collect();

        evaluacion.publicar();
        let update_doc = doc! {
            "$set": {
                "estado": evaluacion.estado.to_string(),
                "examenes": examenes_docs,
            }
        };

        match self
            .get_collection()
            .update_one(
                doc! {
                    "_id": evaluacion.id.to_string()
                },
                update_doc,
            )
            .await
        {
            Ok(result) => {
                if result.matched_count == 0 {
                    error!("No evaluation found with ID: {}", evaluacion.id.to_string());
                    return Err(EvaluacionError::EvaluacionRepositorioError(
                        EvaluacionNoExiste,
                    ));
                }
                Ok(())
            }

            Err(e) => {
                error!("Error updating evaluation: {}", e);
                Err(EvaluacionError::EvaluacionRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
