use crate::mongo::examen::pregunta_dto::PreguntaMongoDTO;
use crate::mongo::examen::write::ExamenMongo;
use crate::mongo::repositorio::MongoRepository;
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

/// Un examen completo tal como se guarda.
#[derive(Deserialize)]
struct FilaExamen {
    #[serde(rename = "_id")]
    id: String,
    titulo: String,
    descripcion: String,
    instrucciones: String,
    activo: String,
    /// Un examen sin preguntas puede no tener el campo.
    #[serde(default)]
    preguntas: Option<bson::Bson>,
}

impl RepositorioExamenLectura for ExamenMongo {
    async fn obtener_examen(&self, id: &str) -> Result<Examen, ExamenError> {
        let fila = self
            .get_collection()
            .clone_with_type::<FilaExamen>()
            .find_one(doc! { "_id": id })
            .await
            .map_err(|e| {
                error!("mongo, leer el examen {id}: {e}");
                ExamenError::ExamenRepositorioError(PersistenciaNoFinalizada)
            })?
            .ok_or(ExamenError::NoEncontrado)?;

        // Una pregunta que no se puede leer es un error con nombre, no se descarta: si no,
        // desaparecía de la evaluación publicada sin que nadie se enterara.
        let preguntas = match fila.preguntas {
            Some(bson::Bson::Array(arr)) => arr
                .into_iter()
                .enumerate()
                .map(|(indice, item)| {
                    bson::from_bson::<PreguntaMongoDTO>(item)
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
            id: ExamenID::new(&fila.id)?,
            titulo: fila.titulo,
            descripcion: fila.descripcion,
            instrucciones: fila.instrucciones,
            estado: EstadoGeneral::from_str(&fila.activo)?,
            preguntas,
        })
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

impl RepositorioExamenListar for ExamenMongo {
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
