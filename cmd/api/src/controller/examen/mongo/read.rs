use crate::controller::examen::dto::PreguntaMongoDTO;
use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::mongo_repository::MongoRepository;
use async_trait::async_trait;
use futures::TryStreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use quizz_core::examen::domain::entity::examen::Examen;
use quizz_core::examen::domain::error::examen::ExamenError;
use quizz_core::examen::domain::error::examen::RepositorioError::{
    LecturaNoFinalizada, PersistenciaNoFinalizada,
};
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::examen::provider::repositorio::{
    RepositorioExamenLectura, RepositorioExamenListar,
};
use quizz_core::examen::use_case::listar_examenes::OutputData;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use serde::Deserialize;
use std::str::FromStr;
use tracing::error;

#[async_trait]
impl RepositorioExamenLectura<ExamenError> for ExamenMongo {
    async fn obtener_examen(&self, id: &str) -> Result<Examen, ExamenError> {
        let filter = doc! { "_id": id };

        match self.get_collection().find_one(filter).await {
            Ok(Some(documento)) => {
                let id_str = documento
                    .get_str("_id")
                    .map_err(|_| ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada))?;

                let titulo = documento
                    .get_str("titulo")
                    .map_err(|_| ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada))?
                    .to_string();

                let descripcion = documento
                    .get_str("descripcion")
                    .map_err(|_| ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada))?
                    .to_string();

                let instrucciones = documento
                    .get_str("instrucciones")
                    .map_err(|_| ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada))?
                    .to_string();

                let estado_str = documento
                    .get_str("activo")
                    .map_err(|_| ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada))?;

                let estado = EstadoGeneral::from_str(estado_str)?;
                let examen_id = ExamenID::new(id_str)?;

                // Una pregunta que no se puede leer es un error con nombre, no se descarta: si
                // no, desaparecía de la evaluación publicada sin que nadie se enterara.
                let preguntas = match documento.get("preguntas") {
                    Some(bson::Bson::Array(arr)) => arr
                        .iter()
                        .enumerate()
                        .map(|(indice, item)| {
                            bson::from_bson::<PreguntaMongoDTO>(item.clone())
                                .map_err(|e| e.to_string())
                                .and_then(|dto| dto.into_entity().map_err(|e| e.to_string()))
                                .map_err(|e| {
                                    error!("examen {id}: pregunta #{indice} ilegible: {e}");
                                    ExamenError::ExamenRepositorioError(LecturaNoFinalizada)
                                })
                        })
                        .collect::<Result<Vec<PreguntaEntity>, _>>()
                        .map(ListaDePreguntas::new)?,
                    _ => ListaDePreguntas::new(Vec::new()),
                };

                Ok(Examen {
                    id: examen_id,
                    titulo,
                    descripcion,
                    instrucciones,
                    estado,
                    preguntas,
                })
            }
            Ok(None) => Err(ExamenError::NoEncontrado),
            Err(e) => {
                error!(
                    "Database error while retrieving examen: id={}, error={}",
                    id, e
                );
                Err(ExamenError::ExamenRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
        }
    }
}

/// Un examen en el listado: sin preguntas (que traen imágenes), solo cuántas tiene.
#[derive(Deserialize)]
struct FilaListaExamen {
    #[serde(rename = "_id")]
    id: String,
    titulo: String,
    descripcion: String,
    instrucciones: String,
    activo: String,
    cantidad_preguntas: i64,
}

#[async_trait]
impl RepositorioExamenListar<ExamenError> for ExamenMongo {
    async fn listar_examenes(&self) -> Result<Vec<OutputData>, ExamenError> {
        let error_lectura = |e: mongodb::error::Error| {
            error!("mongo, listar examenes: {e}");
            ExamenError::ExamenRepositorioError(LecturaNoFinalizada)
        };
        let filas: Vec<FilaListaExamen> = self
            .get_collection()
            .clone_with_type::<FilaListaExamen>()
            .find(doc! {})
            .projection(doc! {
                "titulo": 1,
                "descripcion": 1,
                "instrucciones": 1,
                "activo": 1,
                "cantidad_preguntas": { "$size": { "$ifNull": ["$preguntas", []] } },
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
                titulo: fila.titulo,
                descripcion: fila.descripcion,
                instrucciones: fila.instrucciones,
                estado: fila.activo,
                cantidad_preguntas: usize::try_from(fila.cantidad_preguntas).unwrap_or(0),
            })
            .collect())
    }
}
