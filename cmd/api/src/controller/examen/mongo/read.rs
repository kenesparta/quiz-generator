use crate::controller::examen::dto::PreguntaMongoDTO;
use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::mongo_repository::MongoRepository;
use async_trait::async_trait;
use mongodb::bson;
use mongodb::bson::doc;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use quizz_core::examen::domain::entity::examen::Examen;
use quizz_core::examen::domain::error::examen::ExamenError;
use quizz_core::examen::domain::error::examen::RepositorioError::PersistenciaNoFinalizada;
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::examen::provider::repositorio::RepositorioExamenLectura;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use std::str::FromStr;
use tracing::log::error;

#[async_trait]
impl RepositorioExamenLectura<ExamenError> for ExamenMongo {
    async fn obtener_examen(&self, id: &str) -> Result<Examen, ExamenError> {
        let filter = doc! { "_id": id };

        match self.get_collection().find_one(filter, None).await {
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

                let preguntas = match documento.get("preguntas") {
                    Some(bson::Bson::Array(arr)) => {
                        let entities_result: Result<Vec<PreguntaEntity>, _> = arr
                            .iter()
                            .filter_map(|item| bson::from_bson(item.clone()).ok())
                            .map(|dto: PreguntaMongoDTO| dto.to_entity())
                            .collect();

                        match entities_result {
                            Ok(entities) => ListaDePreguntas::new(entities),
                            Err(e) => {
                                error!("Error converting preguntas to entities: {}", e);
                                return Err(ExamenError::ExamenRepositorioError(
                                    PersistenciaNoFinalizada,
                                ));
                            }
                        }
                    }
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
            Ok(None) => {
                error!("Examen not found with id: {}", id);
                Err(ExamenError::ExamenRepositorioError(
                    PersistenciaNoFinalizada,
                ))
            }
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

impl ExamenMongo {}
