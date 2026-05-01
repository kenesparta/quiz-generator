use crate::controller::mongo_repository::MongoRepository;
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use crate::controller::respuesta::mongo::respuesta_dto::RespuestaDTO;
use actix_web::web;
use async_trait::async_trait;
use futures::StreamExt;
use mongodb;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::respuesta::domain::entity::respuesta::{Estado, Respuesta};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::{
    RepositorioListaRespuestaPostulante, RepositorioListarAsignaciones,
    RepositorioRespuestaLectura, RespositorioRespuestaRevision,
};
use tracing::error;

pub struct RespuestaPorPostulanteMongo {
    client: web::Data<mongodb::Client>,
}

impl RespuestaPorPostulanteMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RespuestaPorPostulanteMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioRespuestaLectura<RespuestaError> for RespuestaPorPostulanteMongo {
    async fn obtener_por_postulante(
        &self,
        respuesta_id: String,
        postulante_id: PostulanteID,
    ) -> Result<Respuesta, RespuestaError> {
        let filter = doc! {
            "postulante_id": postulante_id.to_string(),
            "_id": respuesta_id,
        };

        let respuesta_doc = self.get_collection().find_one(filter).await.map_err(|e| {
            error!(
                "Error finding respuesta by postulante_id {}: {}",
                postulante_id, e
            );
            RespuestaError::RepositorioError
        })?;

        match respuesta_doc {
            Some(doc) => {
                let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                    error!("Error deserializing respuesta document: {}", e);
                    RespuestaError::RepositorioError
                })?;

                Ok(respuesta_dto.into())
            }

            None => Err(RespuestaError::RespuestaNoEncontrada),
        }
    }
}

pub struct RespuestaRevisionMongo {
    client: web::Data<mongodb::Client>,
}

impl RespuestaRevisionMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for RespuestaRevisionMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RespositorioRespuestaRevision<RespuestaError> for RespuestaRevisionMongo {
    async fn obtener_respuesta_revision(
        &self,
        estado: Estado,
    ) -> Result<Vec<Respuesta>, RespuestaError> {
        let filter = doc! {
            "estado": estado.to_string()
        };

        let mut cursor = self.get_collection().find(filter).await.map_err(|e| {
            error!("Error finding respuestas by estado {}: {}", estado, e);
            RespuestaError::RepositorioError
        })?;

        let mut respuestas = Vec::new();

        while cursor.advance().await.map_err(|e| {
            error!("Error advancing cursor: {}", e);
            RespuestaError::RepositorioError
        })? {
            let doc = cursor.deserialize_current().map_err(|e| {
                error!("Error deserializing cursor: {}", e);
                RespuestaError::RepositorioError
            })?;
            let respuesta_dto: RespuestaDTO = bson::from_document(doc).map_err(|e| {
                error!("Error deserializing respuesta document: {}", e);
                RespuestaError::RepositorioError
            })?;
            respuestas.push(respuesta_dto.into());
        }

        Ok(respuestas)
    }
}

pub struct ListaRespuestaPostulanteMongo {
    client: web::Data<mongodb::Client>,
}

impl ListaRespuestaPostulanteMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for ListaRespuestaPostulanteMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioListaRespuestaPostulante<RespuestaError> for ListaRespuestaPostulanteMongo {
    async fn obtener_respuestas_por_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<
        Vec<quizz_core::respuesta::use_case::lista_respuesta_postulante::OutputData>,
        RespuestaError,
    > {
        let filter = doc! {
            "postulante_id": postulante_id.to_string(),
            "estado": {
                "$ne": Estado::Finalizado.to_string()
            }
        };

        let mut cursor = self.get_collection().find(filter).await.map_err(|e| {
            error!(
                "Error finding respuestas by postulante_id {}: {}",
                postulante_id, e
            );
            RespuestaError::RepositorioError
        })?;

        let mut respuestas = Vec::new();

        while cursor.advance().await.map_err(|e| {
            error!("Error advancing cursor: {}", e);
            RespuestaError::RepositorioError
        })? {
            let doc = cursor.deserialize_current().map_err(|e| {
                error!("Error deserializing cursor: {}", e);
                RespuestaError::RepositorioError
            })?;

            let respuesta_id = doc
                .get_str("_id")
                .map_err(|_| RespuestaError::RepositorioError)?
                .to_string();

            let estado = doc
                .get_str("estado")
                .map_err(|_| RespuestaError::RepositorioError)?
                .to_string();

            let evaluacion_doc = doc
                .get_document("evaluacion")
                .map_err(|_| RespuestaError::RepositorioError)?;

            let nombre_evaluacion = evaluacion_doc
                .get_str("nombre")
                .map_err(|_| RespuestaError::RepositorioError)?
                .to_string();

            let descripcion_evaluacion = evaluacion_doc
                .get_str("descripcion")
                .map_err(|_| RespuestaError::RepositorioError)?
                .to_string();

            respuestas.push(
                quizz_core::respuesta::use_case::lista_respuesta_postulante::OutputData {
                    respuesta_id,
                    nombre_evaluacion,
                    descripcion_evaluacion,
                    estado,
                },
            );
        }

        Ok(respuestas)
    }
}

pub struct ListarAsignacionesMongo {
    client: web::Data<mongodb::Client>,
}

impl ListarAsignacionesMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for ListarAsignacionesMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioListarAsignaciones<RespuestaError> for ListarAsignacionesMongo {
    async fn listar(
        &self,
        postulante_id: Option<String>,
        evaluacion_id: Option<String>,
    ) -> Result<Vec<quizz_core::respuesta::use_case::listar_asignaciones::OutputData>, RespuestaError>
    {
        let mut match_doc = bson::Document::new();
        if let Some(pid) = postulante_id {
            match_doc.insert("postulante_id", pid);
        }
        if let Some(eid) = evaluacion_id {
            match_doc.insert("evaluacion._id", eid);
        }

        let pipeline = vec![
            doc! { "$match": match_doc },
            doc! {
                "$lookup": {
                    "from": "postulante",
                    "localField": "postulante_id",
                    "foreignField": "_id",
                    "as": "postulante",
                }
            },
            doc! {
                "$unwind": {
                    "path": "$postulante",
                    "preserveNullAndEmptyArrays": true,
                }
            },
            doc! {
                "$project": {
                    "_id": 1,
                    "estado": 1,
                    "fecha_tiempo_inicio": 1,
                    "fecha_tiempo_fin": 1,
                    "evaluacion._id": 1,
                    "evaluacion.nombre": 1,
                    "evaluacion.descripcion": 1,
                    "postulante._id": 1,
                    "postulante.documento": 1,
                    "postulante.nombre": 1,
                    "postulante.primer_apellido": 1,
                    "postulante.segundo_apellido": 1,
                }
            },
        ];

        let mut cursor = self
            .get_collection()
            .aggregate(pipeline)
            .await
            .map_err(|e| {
                error!("Error en aggregate de asignaciones: {}", e);
                RespuestaError::RepositorioError
            })?;

        let mut asignaciones = Vec::new();

        while let Some(result) = cursor.next().await {
            let doc = result.map_err(|e| {
                error!("Error iterando cursor de asignaciones: {}", e);
                RespuestaError::RepositorioError
            })?;

            let respuesta_id = doc.get_str("_id").unwrap_or_default().to_string();
            let estado = doc.get_str("estado").unwrap_or_default().to_string();
            let fecha_tiempo_inicio =
                doc.get_str("fecha_tiempo_inicio").unwrap_or_default().to_string();
            let fecha_tiempo_fin =
                doc.get_str("fecha_tiempo_fin").unwrap_or_default().to_string();

            let evaluacion_doc = doc.get_document("evaluacion").map_err(|_| {
                error!("Asignacion sin evaluacion para respuesta_id={}", respuesta_id);
                RespuestaError::RepositorioError
            })?;

            let evaluacion_id =
                evaluacion_doc.get_str("_id").unwrap_or_default().to_string();
            let evaluacion_nombre =
                evaluacion_doc.get_str("nombre").unwrap_or_default().to_string();
            let evaluacion_descripcion = evaluacion_doc
                .get_str("descripcion")
                .unwrap_or_default()
                .to_string();

            let (
                postulante_id_out,
                postulante_documento,
                postulante_nombre,
                postulante_primer_apellido,
                postulante_segundo_apellido,
            ) = match doc.get_document("postulante") {
                Ok(p) => (
                    p.get_str("_id").unwrap_or_default().to_string(),
                    p.get_str("documento").unwrap_or_default().to_string(),
                    p.get_str("nombre").unwrap_or_default().to_string(),
                    p.get_str("primer_apellido").unwrap_or_default().to_string(),
                    p.get_str("segundo_apellido").unwrap_or_default().to_string(),
                ),
                Err(_) => (
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ),
            };

            asignaciones.push(
                quizz_core::respuesta::use_case::listar_asignaciones::OutputData {
                    respuesta_id,
                    estado,
                    fecha_tiempo_inicio,
                    fecha_tiempo_fin,
                    evaluacion_id,
                    evaluacion_nombre,
                    evaluacion_descripcion,
                    postulante_id: postulante_id_out,
                    postulante_documento,
                    postulante_nombre,
                    postulante_primer_apellido,
                    postulante_segundo_apellido,
                },
            );
        }

        Ok(asignaciones)
    }
}
