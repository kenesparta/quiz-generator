use crate::controller::postulante::mongo::constantes::{
    MAIN_DATABASE_NAME, POSTULANTE_COLLECTION_NAME,
};
use actix_web::web;
use async_trait::async_trait;
use futures::StreamExt;
use mongodb::{
    Collection,
    bson::{Document, doc},
};
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::documento::Documento;
use quizz_core::postulante::domain::value_object::genero::Genero;
use quizz_core::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::domain::value_object::nombre::Nombre;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteLectura;
use std::str::FromStr;
use tracing::log::error;

pub struct PostulanteReadMongo {
    client: web::Data<mongodb::Client>,
}

impl PostulanteReadMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        PostulanteReadMongo { client }
    }

    fn get_collection(&self) -> Collection<Document> {
        self.client
            .database(MAIN_DATABASE_NAME)
            .collection::<Document>(POSTULANTE_COLLECTION_NAME)
    }
}

#[async_trait]
impl RepositorioPostulanteLectura<PostulanteError> for PostulanteReadMongo {
    async fn obtener_postulante_por_documento(
        &self,
        documento: Documento,
    ) -> Result<Postulante, PostulanteError> {
        let doc_string = documento.to_string();
        let filter = doc! { "documento": doc_string.clone() };

        match self.get_collection().find_one(filter, None).await {
            Ok(Some(doc)) => {
                let id = match doc.get("id") {
                    Some(bson_id) => bson_id.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let documento = match doc.get("documento") {
                    Some(doc_bson) => doc_bson.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let nombre = match doc.get("nombre") {
                    Some(bson_nombre) => bson_nombre.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let primer_apellido = match doc.get("primer_apellido") {
                    Some(bson_apellido) => bson_apellido.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let segundo_apellido = match doc.get("segundo_apellido") {
                    Some(bson_apellido) => bson_apellido.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let fecha_nacimiento = match doc.get("fecha_nacimiento") {
                    Some(bson_fecha) => bson_fecha.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let grado_instruccion = match doc.get("grado_instruccion") {
                    Some(bson_grado) => bson_grado.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let genero = match doc.get("genero") {
                    Some(bson_genero) => bson_genero.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteError::PostulanteRepositorioError(
                            RepositorioError::LecturaNoFinalizada,
                        ));
                    }
                };

                let id = PostulanteID::new(&id)?;
                let documento = Documento::new(&documento)?;
                let nombre_completo = Nombre::new(nombre, primer_apellido, segundo_apellido)?;
                let fecha_nacimiento = FechaNacimiento::new(&fecha_nacimiento)?;
                let grado_instruccion = GradoInstruccion::from_str(&grado_instruccion)?;
                let genero = Genero::from_str(&genero)?;

                Ok(Postulante {
                    id,
                    documento,
                    nombre_completo,
                    fecha_nacimiento,
                    grado_instruccion,
                    genero,
                    password: None,
                })
            }
            Ok(None) => {
                error!("No postulante found with documento: {}", doc_string);
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::RegistroNoEncontrado,
                ))
            }
            Err(e) => {
                error!(
                    "Database error while fetching postulante with documento={}, error={}",
                    doc_string, e
                );
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::LecturaNoFinalizada,
                ))
            }
        }
    }

    async fn obtener_postulante_por_id(
        &self,
        _postulante_id: PostulanteID,
    ) -> Result<Postulante, PostulanteError> {
        todo!()
    }

    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
        match self.get_collection().find(None, None).await {
            Ok(mut cursor) => {
                let mut docs = Vec::new();
                while let Some(result) = cursor.next().await {
                    match result {
                        Ok(doc) => docs.push(doc),
                        Err(e) => {
                            error!("Error fetching document from cursor: {}", e);
                            return Err(PostulanteError::PostulanteRepositorioError(
                                RepositorioError::LecturaNoFinalizada,
                            ));
                        }
                    }
                }

                let postulantes: Vec<Postulante> = docs
                    .into_iter()
                    .filter_map(|doc| {
                        match (|| {
                            let id = match doc.get("id") {
                                Some(bson_id) => bson_id
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let documento = match doc.get("documento") {
                                Some(doc_bson) => doc_bson
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let nombre = match doc.get("nombre") {
                                Some(bson_nombre) => bson_nombre
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let primer_apellido = match doc.get("primer_apellido") {
                                Some(bson_apellido) => bson_apellido
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let segundo_apellido = match doc.get("segundo_apellido") {
                                Some(bson_apellido) => bson_apellido
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let fecha_nacimiento = match doc.get("fecha_nacimiento") {
                                Some(bson_fecha) => bson_fecha
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let grado_instruccion = match doc.get("grado_instruccion") {
                                Some(bson_grado) => bson_grado
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let genero = match doc.get("genero") {
                                Some(bson_genero) => bson_genero
                                    .as_str()
                                    .ok_or_else(|| {
                                        PostulanteError::PostulanteRepositorioError(
                                            RepositorioError::LecturaNoFinalizada,
                                        )
                                    })?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let id = PostulanteID::new(&id)?;
                            let documento = Documento::new(&documento)?;
                            let nombre_completo =
                                Nombre::new(nombre, primer_apellido, segundo_apellido)?;
                            let fecha_nacimiento = FechaNacimiento::new(&fecha_nacimiento)?;
                            let grado_instruccion = GradoInstruccion::from_str(&grado_instruccion)?;
                            let genero = Genero::from_str(&genero)?;

                            Ok(Postulante {
                                id,
                                documento,
                                nombre_completo,
                                fecha_nacimiento,
                                grado_instruccion,
                                genero,
                                password: None,
                            })
                        })() {
                            Ok(postulante) => Some(postulante),
                            Err(e) => {
                                error!(
                                    "Error al convertir documento MongoDB a entidad Postulante: {}",
                                    e
                                );
                                None
                            }
                        }
                    })
                    .collect();

                Ok(postulantes)
            }
            Err(e) => {
                error!(
                    "Error de base de datos al obtener la lista de postulantes: {}",
                    e
                );
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::LecturaNoFinalizada,
                ))
            }
        }
    }
}
