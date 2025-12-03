use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::mongo_repository::MongoRepository;
use crate::controller::postulante::mongo::write::PostulanteMongo;
use crate::controller::respuesta::dto::{EvaluacionMongoDTO, RespuestaMongoDTO};
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::respuesta::domain::entity::pregunta::Puntaje;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, RespuestaEvaluacion, Revision};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use quizz_core::respuesta::provider::repositorio::{
    RepositorioRespuestaEscritura, RespositorioFinalizarEvaluacion,
};
use std::str::FromStr;

pub struct RespuestaEvaluacionMongo {
    client: web::Data<mongodb::Client>,
    repositorio_evaluacion: EvaluacionMongo,
    reposiorio_postulante: PostulanteMongo,
}

impl RespuestaEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        let repositorio_evaluacion = EvaluacionMongo::new(client.clone());
        let reposiorio_postulante = PostulanteMongo::new(client.clone());
        Self {
            client,
            repositorio_evaluacion,
            reposiorio_postulante,
        }
    }
}

impl MongoRepository for RespuestaEvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioRespuestaEscritura<RespuestaError> for RespuestaEvaluacionMongo {
    async fn asignar_evaluacion(
        &self,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> Result<(), RespuestaError> {
        let existing_respuesta = self
            .get_collection()
            .find_one(
                doc! {
                    "evaluacion._id": evaluacion_id.to_string(),
                    "postulante_id": postulante_id.to_string(),
                },
                None,
            )
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if existing_respuesta.is_some() {
            return Err(RespuestaError::EvaluacionAlreadyAssigned);
        }

        let postulante_exists = self
            .reposiorio_postulante
            .get_collection()
            .find_one(doc! { "_id": postulante_id.to_string() }, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if postulante_exists.is_none() {
            return Err(RespuestaError::PostulanteRespuestaNotFound);
        }

        let evaluacion_doc = self
            .repositorio_evaluacion
            .get_collection()
            .find_one(doc! { "_id": evaluacion_id.to_string() }, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        let evaluacion_document =
            evaluacion_doc.ok_or_else(|| RespuestaError::EvaluacionRespuestaNotFound)?;

        let evaluacion: EvaluacionMongoDTO =
            bson::from_document(evaluacion_document).map_err(|_| RespuestaError::DatabaseError)?;

        let respuesta_dto = RespuestaMongoDTO {
            id: RespuestaID::new_v4().to_string(),
            evaluacion,
            postulante_id: postulante_id.to_string(),
            fecha_tiempo_inicio: String::new(),
            fecha_tiempo_fin: String::new(),
            estado: Estado::Creado.to_string(),
            revision: Revision::SinIniciar.to_string(),
        };

        let respuesta_doc =
            bson::to_document(&respuesta_dto).map_err(|_| RespuestaError::DatabaseError)?;

        self.get_collection()
            .insert_one(respuesta_doc, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }

    async fn responder_evaluacion(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<(), RespuestaError> {
        let filter = doc! {
            "_id": &respuesta_evaluacion.id.to_string(),
            "evaluacion._id": &respuesta_evaluacion.evaluacion_id,
            "evaluacion.examenes._id": &respuesta_evaluacion.examen_id,
            "evaluacion.examenes.preguntas._id": &respuesta_evaluacion.pregunta_id
        };

        let update = doc! {
            "$set": {
                "evaluacion.examenes.$[examen].preguntas.$[pregunta].respuestas": &respuesta_evaluacion.respuestas,
                "evaluacion.examenes.$[examen].preguntas.$[pregunta].puntos": &respuesta_evaluacion.puntos,
            }
        };

        let array_filters = vec![
            doc! { "examen._id": &respuesta_evaluacion.examen_id },
            doc! { "pregunta._id": &respuesta_evaluacion.pregunta_id },
        ];

        let mut options = mongodb::options::UpdateOptions::default();
        options.array_filters = Some(array_filters);

        let _result = self
            .get_collection()
            .update_one(filter, update, Some(options))
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }

    async fn obtener_puntaje(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<Puntaje, RespuestaError> {
        use std::collections::HashMap;

        let filter = doc! {
            "_id": &respuesta_evaluacion.id.to_string(),
        };

        let result = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?
            .ok_or(RespuestaError::DatabaseError)?;

        // Navigate through the nested structure to find the specific question
        let evaluacion = result
            .get_document("evaluacion")
            .map_err(|_| RespuestaError::DatabaseError)?;

        let examenes = evaluacion
            .get_array("examenes")
            .map_err(|_| RespuestaError::DatabaseError)?;

        // Find the specific examen by ID
        let examen_doc = examenes
            .iter()
            .filter_map(|e| e.as_document())
            .find(|doc| {
                doc.get_str("_id")
                    .map(|id| id == respuesta_evaluacion.examen_id)
                    .unwrap_or(false)
            })
            .ok_or(RespuestaError::DatabaseError)?;

        let preguntas = examen_doc
            .get_array("preguntas")
            .map_err(|_| RespuestaError::DatabaseError)?;

        // Find the specific pregunta by ID
        let pregunta_doc = preguntas
            .iter()
            .filter_map(|p| p.as_document())
            .find(|doc| {
                doc.get_str("_id")
                    .map(|id| id == respuesta_evaluacion.pregunta_id)
                    .unwrap_or(false)
            })
            .ok_or(RespuestaError::DatabaseError)?;

        let puntaje_doc = pregunta_doc
            .get_document("puntaje")
            .map_err(|_| RespuestaError::DatabaseError)?;

        // Convert BSON document to HashMap<String, u32>
        let mut puntaje: Puntaje = HashMap::new();
        for (key, value) in puntaje_doc.iter() {
            let puntos = match value {
                bson::Bson::Document(doc) => {
                    // Handle MongoDB's NumberLong format {"$numberLong": "1"}
                    if let Ok(num_str) = doc.get_str("$numberLong") {
                        num_str
                            .parse::<u32>()
                            .map_err(|_| RespuestaError::DatabaseError)?
                    } else {
                        return Err(RespuestaError::DatabaseError);
                    }
                }
                bson::Bson::Int32(n) => *n as u32,
                bson::Bson::Int64(n) => *n as u32,
                _ => return Err(RespuestaError::DatabaseError),
            };
            puntaje.insert(key.to_string(), puntos);
        }

        Ok(puntaje)
    }
}

pub struct RespositorioFinalizarEvaluacionMongo {
    client: web::Data<mongodb::Client>,
    repositorio_evaluacion: EvaluacionMongo,
}

impl RespositorioFinalizarEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        let repositorio_evaluacion = EvaluacionMongo::new(client.clone());
        Self {
            client,
            repositorio_evaluacion,
        }
    }
}

impl MongoRepository for RespositorioFinalizarEvaluacionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RespositorioFinalizarEvaluacion<RespuestaError> for RespositorioFinalizarEvaluacionMongo {
    async fn sumar_puntos(&self, evaluacion_id: String) -> Result<(), RespuestaError> {
        let filter = doc! {
            "_id": &evaluacion_id,
        };

        let result = self
            .get_collection()
            .find_one(filter.clone(), None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?
            .ok_or(RespuestaError::DatabaseError)?;

        let evaluacion = result
            .get_document("evaluacion")
            .map_err(|_| RespuestaError::DatabaseError)?;

        let examenes = evaluacion
            .get_array("examenes")
            .map_err(|_| RespuestaError::DatabaseError)?;

        // Calculate puntos_obtenidos for each examen
        for examen_bson in examenes.iter() {
            if let bson::Bson::Document(examen_doc) = examen_bson {
                let examen_id = examen_doc
                    .get_str("_id")
                    .map_err(|_| RespuestaError::DatabaseError)?;

                let mut examen_total_puntos: u32 = 0;

                if let Ok(preguntas) = examen_doc.get_array("preguntas") {
                    for pregunta_bson in preguntas.iter() {
                        if let bson::Bson::Document(pregunta_doc) = pregunta_bson {
                            if let Some(puntos_value) = pregunta_doc.get("puntos") {
                                let puntos = match puntos_value {
                                    bson::Bson::Int32(n) => *n as u32,
                                    bson::Bson::Int64(n) => *n as u32,
                                    bson::Bson::Double(n) => *n as u32,
                                    _ => 0,
                                };
                                examen_total_puntos += puntos;
                            }
                        }
                    }
                }

                let update = doc! {
                    "$set": {
                        "evaluacion.examenes.$[examen].puntos_obtenidos": examen_total_puntos as i64,
                    }
                };

                let array_filters = vec![doc! { "examen._id": examen_id }];

                let mut options = mongodb::options::UpdateOptions::default();
                options.array_filters = Some(array_filters);

                self.get_collection()
                    .update_one(filter.clone(), update, Some(options))
                    .await
                    .map_err(|_| RespuestaError::DatabaseError)?;
            }
        }

        Ok(())
    }

    async fn obtener_estado(&self, evaluacion_id: String) -> Result<Estado, RespuestaError> {
        let filter = doc! {
            "_id": &evaluacion_id,
        };

        let result = self
            .get_collection()
            .find_one(filter, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?
            .ok_or(RespuestaError::DatabaseError)?;

        let estado_str = result
            .get_str("estado")
            .map_err(|_| RespuestaError::DatabaseError)?;

        let estado = Estado::from_str(estado_str).map_err(|_| RespuestaError::DatabaseError)?;

        Ok(estado)
    }

    async fn alterar_estado(&self, evaluacion_id: String) -> Result<(), RespuestaError> {
        let filter = doc! {
            "_id": &evaluacion_id,
        };

        let fecha_actual = chrono::Utc::now().to_rfc3339();

        let update = doc! {
            "$set": {
                "estado": Estado::Finalizado.to_string(),
                "fecha_tiempo_fin": fecha_actual,
            }
        };

        self.get_collection()
            .update_one(filter, update, None)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }
}
