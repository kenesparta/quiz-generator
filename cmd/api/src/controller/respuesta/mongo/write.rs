use crate::controller::evaluacion::mongo::write::EvaluacionMongo;
use crate::controller::mongo_repository::MongoRepository;
use crate::controller::postulante::mongo::write::PostulanteMongo;
use crate::controller::respuesta::dto::{EvaluacionMongoDTO, RespuestaMongoDTO};
use crate::controller::respuesta::mongo::constantes::RESPUESTA_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};
use log::error;
use mongodb::bson;
use mongodb::bson::{Bson, Document, doc};
use quizz_common::domain::value_objects::zona_horaria::formatear_rfc3339;
use quizz_core::evaluacion::value_object::id::EvaluacionID;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use quizz_core::respuesta::domain::entity::pregunta::{PreguntaACorregir, Puntaje};
use quizz_core::respuesta::domain::entity::respuesta::{Estado, RespuestaEvaluacion, Revision};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::domain::value_object::id::RespuestaID;
use quizz_core::respuesta::provider::repositorio::{
    RepositorioEstadoRespuesta, RepositorioRespuestaEscritura,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::str::FromStr;

/// Registra el error del driver (que de otro modo se perdería) y lo traduce a un error del
/// dominio sin detalles.
fn error_bd(contexto: &'static str) -> impl FnOnce(mongodb::error::Error) -> RespuestaError {
    move |e| {
        error!("mongo, {contexto}: {e}");
        RespuestaError::DatabaseError
    }
}

/// Filtro de una hoja de respuestas de un postulante: toda lectura o escritura de la hoja en
/// nombre del postulante lo usa, así nunca toca la hoja de otro.
fn filtro_hoja(id: &RespuestaID, postulante_id: &PostulanteID) -> Document {
    doc! { "_id": id.to_string(), "postulante_id": postulante_id.to_string() }
}

/// La parte de la hoja que hace falta para corregir (sin enunciados ni imágenes).
#[derive(Deserialize)]
struct HojaParaCorregir {
    estado: String,
    evaluacion: EvaluacionParaCorregir,
}

#[derive(Deserialize)]
struct EvaluacionParaCorregir {
    examenes: Vec<ExamenParaCorregir>,
}

#[derive(Deserialize)]
struct ExamenParaCorregir {
    #[serde(rename = "_id")]
    id: String,
    #[serde(default)]
    preguntas: Vec<PreguntaParaCorregir>,
}

#[derive(Deserialize)]
struct PreguntaParaCorregir {
    #[serde(rename = "_id")]
    id: String,
    tipo_de_pregunta: String,
    #[serde(default)]
    alternativas: HashMap<String, String>,
    #[serde(default)]
    puntaje: HashMap<String, Bson>,
}

/// Puntos guardados como entero de 32 o 64 bits; un valor negativo o fuera de rango es un dato
/// corrupto, no se trunca.
fn leer_puntos(valor: &Bson) -> Option<u32> {
    match valor {
        Bson::Int32(n) => u32::try_from(*n).ok(),
        Bson::Int64(n) => u32::try_from(*n).ok(),
        _ => None,
    }
}

pub struct RespuestaEvaluacionMongo {
    client: web::Data<mongodb::Database>,
    repositorio_evaluacion: EvaluacionMongo,
    reposiorio_postulante: PostulanteMongo,
}

impl RespuestaEvaluacionMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
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

    fn get_db(&self) -> &web::Data<mongodb::Database> {
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
            .find_one(doc! {
                "evaluacion._id": evaluacion_id.to_string(),
                "postulante_id": postulante_id.to_string(),
            })
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if existing_respuesta.is_some() {
            return Err(RespuestaError::EvaluacionAlreadyAssigned);
        }

        let postulante_exists = self
            .reposiorio_postulante
            .get_collection()
            .find_one(doc! { "_id": postulante_id.to_string() })
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        if postulante_exists.is_none() {
            return Err(RespuestaError::PostulanteRespuestaNotFound);
        }

        let evaluacion_doc = self
            .repositorio_evaluacion
            .get_collection()
            .find_one(doc! { "_id": evaluacion_id.to_string() })
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        let evaluacion_document =
            evaluacion_doc.ok_or(RespuestaError::EvaluacionRespuestaNotFound)?;

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
            .insert_one(respuesta_doc)
            .await
            .map_err(|_| RespuestaError::DatabaseError)?;

        Ok(())
    }

    async fn responder_evaluacion(
        &self,
        contestacion: &RespuestaEvaluacion,
    ) -> Result<bool, RespuestaError> {
        let mut filtro = filtro_hoja(&contestacion.id, &contestacion.postulante_id);
        filtro.insert("estado", Estado::EnProceso.to_string());
        filtro.insert("evaluacion.examenes._id", &contestacion.examen_id);
        filtro.insert(
            "evaluacion.examenes.preguntas._id",
            &contestacion.pregunta_id,
        );

        let actualizacion = doc! {
            "$set": {
                "evaluacion.examenes.$[examen].preguntas.$[pregunta].respuestas": &contestacion.respuestas,
                "evaluacion.examenes.$[examen].preguntas.$[pregunta].puntos": i64::from(contestacion.puntos),
            }
        };
        let array_filters = vec![
            doc! { "examen._id": &contestacion.examen_id },
            doc! { "pregunta._id": &contestacion.pregunta_id },
        ];

        let resultado = self
            .get_collection()
            .update_one(filtro, actualizacion)
            .array_filters(array_filters)
            .await
            .map_err(error_bd("guardar la contestacion"))?;
        Ok(resultado.matched_count > 0)
    }

    async fn obtener_pregunta(
        &self,
        contestacion: &RespuestaEvaluacion,
    ) -> Result<PreguntaACorregir, RespuestaError> {
        let documento = self
            .get_collection()
            .find_one(filtro_hoja(&contestacion.id, &contestacion.postulante_id))
            .projection(doc! {
                "estado": 1,
                "evaluacion.examenes._id": 1,
                "evaluacion.examenes.preguntas._id": 1,
                "evaluacion.examenes.preguntas.tipo_de_pregunta": 1,
                "evaluacion.examenes.preguntas.alternativas": 1,
                "evaluacion.examenes.preguntas.puntaje": 1,
            })
            .await
            .map_err(error_bd("leer la pregunta a corregir"))?
            .ok_or(RespuestaError::RespuestaNoEncontrada)?;

        let hoja: HojaParaCorregir = bson::from_document(documento).map_err(|e| {
            error!("hoja {} ilegible: {e}", contestacion.id);
            RespuestaError::DatabaseError
        })?;

        let pregunta = hoja
            .evaluacion
            .examenes
            .into_iter()
            .find(|examen| examen.id == contestacion.examen_id)
            .ok_or(RespuestaError::ExamenNotFound)?
            .preguntas
            .into_iter()
            .find(|pregunta| pregunta.id == contestacion.pregunta_id)
            .ok_or(RespuestaError::PreguntaNotFound)?;

        let dato_corrupto = |campo: &str| {
            error!("hoja {}: {campo} no valido", contestacion.id);
            RespuestaError::DatabaseError
        };
        let puntaje = pregunta
            .puntaje
            .iter()
            .map(|(clave, valor)| Some((clave.clone(), leer_puntos(valor)?)))
            .collect::<Option<Puntaje>>()
            .ok_or_else(|| dato_corrupto("puntaje"))?;

        Ok(PreguntaACorregir {
            estado: Estado::from_str(&hoja.estado).map_err(|_| dato_corrupto("estado"))?,
            tipo_de_pregunta: TipoPregunta::from_str(&pregunta.tipo_de_pregunta)
                .map_err(|_| dato_corrupto("tipo_de_pregunta"))?,
            alternativas: pregunta.alternativas,
            puntaje,
        })
    }
}

/// Estado de las hojas de respuestas: lecturas y transiciones siempre filtradas por el dueño.
pub struct EstadoRespuestaMongo {
    client: web::Data<mongodb::Database>,
}

impl EstadoRespuestaMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        Self { client }
    }
}

impl MongoRepository for EstadoRespuestaMongo {
    fn get_collection_name(&self) -> &str {
        RESPUESTA_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

#[async_trait]
impl RepositorioEstadoRespuesta<RespuestaError> for EstadoRespuestaMongo {
    async fn obtener_estado(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
    ) -> Result<Option<Estado>, RespuestaError> {
        let Some(documento) = self
            .get_collection()
            .find_one(filtro_hoja(id, postulante_id))
            .projection(doc! { "estado": 1 })
            .await
            .map_err(error_bd("leer el estado"))?
        else {
            return Ok(None);
        };

        documento
            .get_str("estado")
            .ok()
            .and_then(|estado| Estado::from_str(estado).ok())
            .map(Some)
            .ok_or_else(|| {
                error!("hoja {id}: estado no valido");
                RespuestaError::DatabaseError
            })
    }

    async fn transicionar(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
        desde: Estado,
        hacia: Estado,
        fecha: DateTime<FixedOffset>,
    ) -> Result<bool, RespuestaError> {
        let campo_fecha = match hacia {
            Estado::EnProceso => "fecha_tiempo_inicio",
            Estado::Finalizado => "fecha_tiempo_fin",
            Estado::Creado => return Err(RespuestaError::TransicionNoAplicada),
        };

        // El estado esperado va en el filtro: comprobar y escribir es una sola operación, así
        // dos peticiones simultáneas no pueden aplicar la misma transición dos veces.
        let mut filtro = filtro_hoja(id, postulante_id);
        filtro.insert("estado", desde.to_string());
        let actualizacion = doc! {
            "$set": { "estado": hacia.to_string(), campo_fecha: formatear_rfc3339(&fecha) }
        };

        let resultado = self
            .get_collection()
            .update_one(filtro, actualizacion)
            .await
            .map_err(error_bd("cambiar el estado"))?;
        Ok(resultado.matched_count > 0)
    }
}
