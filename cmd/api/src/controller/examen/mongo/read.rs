use crate::controller::examen::mongo::write::ExamenMongo;
use crate::controller::mongo_repository::MongoRepository;
use async_trait::async_trait;
use mongodb::bson::Bson::Document;
use mongodb::bson::doc;
use quizz_common::domain::value_objects::estado::EstadoGeneral;
use quizz_core::examen::domain::entity::examen::Examen;
use quizz_core::examen::domain::error::examen::ExamenError;
use quizz_core::examen::domain::error::examen::RepositorioError::PersistenciaNoFinalizada;
use quizz_core::examen::domain::value_object::id::ExamenID;
use quizz_core::examen::provider::repositorio::RepositorioExamenLectura;
use quizz_core::pregunta::domain::entity::pregunta::PreguntaEntity;
use quizz_core::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use quizz_core::pregunta::domain::value_object::etiqueta::Etiqueta;
use quizz_core::pregunta::domain::value_object::id::PreguntaID;
use quizz_core::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;
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

                let preguntas = match documento.get_array("preguntas") {
                    Ok(preguntas_array) => {
                        let mut preguntas_vec = Vec::new();

                        for pregunta_bson in preguntas_array {
                            if let Document(pregunta_doc) = pregunta_bson {
                                match self.documento_to_pregunta(pregunta_doc.clone()) {
                                    Ok(pregunta) => preguntas_vec.push(pregunta),
                                    Err(e) => {
                                        error!("Error converting document to pregunta: {}", e);
                                        return Err(ExamenError::ExamenRepositorioError(
                                            PersistenciaNoFinalizada,
                                        ));
                                    }
                                }
                            }
                        }
                        ListaDePreguntas::new(preguntas_vec)
                    }
                    Err(_) => ListaDePreguntas::new(Vec::new()),
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

impl ExamenMongo {
    fn documento_to_pregunta(
        &self,
        documento: mongodb::bson::Document,
    ) -> Result<PreguntaEntity, Box<dyn std::error::Error>> {
        let id_str = documento.get_str("id")?;
        let contenido = documento.get_str("contenido")?.to_string();
        let etiqueta_str = documento.get_str("etiqueta")?;
        let tipo_pregunta_str = documento.get_str("tipo_de_pregunta")?;
        let imagen_ref = documento.get_str("imagen_ref").ok().map(|s| s.to_string());

        // Parse alternativas from document
        let alternativas_doc = documento.get_document("alternativas")?;
        let mut alternativas = HashMap::new();
        for (key, value) in alternativas_doc {
            if let mongodb::bson::Bson::String(val) = value {
                alternativas.insert(key.clone(), val.clone());
            }
        }

        let puntaje_doc = documento.get_document("puntaje")?;
        let mut puntaje = HashMap::new();
        for (key, value) in puntaje_doc {
            match value {
                mongodb::bson::Bson::Int32(val) => {
                    puntaje.insert(key.clone(), *val as u32);
                }
                mongodb::bson::Bson::Int64(val) => {
                    puntaje.insert(key.clone(), *val as u32);
                }
                _ => {}
            }
        }

        let id = PreguntaID::new(id_str)?;
        let etiqueta = Etiqueta::from_str(etiqueta_str)?;
        let tipo_de_pregunta = TipoPregunta::from_str(tipo_pregunta_str)?;

        Ok(PreguntaEntity {
            id,
            contenido,
            imagen_ref,
            etiqueta,
            tipo_de_pregunta,
            alternativas,
            puntaje,
        })
    }
}
