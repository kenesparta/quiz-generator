use crate::controller::mongo_repository::MongoRepository;
use crate::controller::postulante::mongo::constantes::POSTULANTE_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use futures::StreamExt;
use log::error;
use mongodb::bson::doc;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;
use quizz_common::domain::value_objects::fecha_registro::FechaRegistro;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::documento::Documento;
use quizz_core::postulante::domain::value_object::genero::Genero;
use quizz_core::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::domain::value_object::nombre::Nombre;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteLectura;
use std::str::FromStr;

pub struct PostulanteReadMongo {
    client: web::Data<mongodb::Client>,
}

impl PostulanteReadMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        PostulanteReadMongo { client }
    }
}

impl MongoRepository for PostulanteReadMongo {
    fn get_collection_name(&self) -> &str {
        POSTULANTE_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
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
        let sort = doc! { "fecha_registro": 1 };

        match self.get_collection().find_one(filter).sort(sort).await {
            Ok(Some(doc)) => {
                let id = match doc.get("_id") {
                    Some(doc_bson) => doc_bson.as_str().unwrap_or_default().to_string(),
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
                    Some(bson_fecha) => {
                        let dt = bson_fecha.as_datetime().ok_or(
                            PostulanteError::PostulanteRepositorioError(
                                RepositorioError::LecturaNoFinalizada,
                            ),
                        )?;
                        {
                            let millis = dt.timestamp_millis();
                            let secs = millis / 1000;
                            let nanos = ((millis % 1000) * 1_000_000) as u32;
                            let naive = chrono::DateTime::from_timestamp(secs, nanos)
                                .unwrap_or_default()
                                .naive_utc();
                            naive.format("%Y-%m-%d %H:%M:%S").to_string()
                        }
                    },
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

                let fecha_registro_str = match doc.get("fecha_registro") {
                    Some(bson_fecha) => {
                        let dt = bson_fecha.as_datetime().ok_or(
                            PostulanteError::PostulanteRepositorioError(
                                RepositorioError::LecturaNoFinalizada,
                            ),
                        )?;
                        {
                            let millis = dt.timestamp_millis();
                            let secs = millis / 1000;
                            let nanos = ((millis % 1000) * 1_000_000) as u32;
                            let naive = chrono::DateTime::from_timestamp(secs, nanos)
                                .unwrap_or_default()
                                .naive_utc();
                            naive.format("%Y-%m-%d %H:%M:%S").to_string()
                        }
                    },
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
                let fecha_registro = FechaRegistro::new(&fecha_registro_str)?;

                Ok(Postulante {
                    id,
                    documento,
                    nombre_completo,
                    fecha_nacimiento,
                    grado_instruccion,
                    genero,
                    password: None,
                    fecha_registro,
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
        postulante_id: PostulanteID,
    ) -> Result<Postulante, PostulanteError> {
        let postulante_id = postulante_id.to_string();
        let filter = doc! { "_id": postulante_id.clone() };

        match self.get_collection().find_one(filter).await {
            Ok(Some(doc)) => {
                let id = match doc.get("_id") {
                    Some(doc_bson) => doc_bson.as_str().unwrap_or_default().to_string(),
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
                    Some(bson_fecha) => {
                        let dt = bson_fecha.as_datetime().ok_or(
                            PostulanteError::PostulanteRepositorioError(
                                RepositorioError::LecturaNoFinalizada,
                            ),
                        )?;
                        {
                            let millis = dt.timestamp_millis();
                            let secs = millis / 1000;
                            let nanos = ((millis % 1000) * 1_000_000) as u32;
                            let naive = chrono::DateTime::from_timestamp(secs, nanos)
                                .unwrap_or_default()
                                .naive_utc();
                            naive.format("%Y-%m-%d %H:%M:%S").to_string()
                        }
                    },
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

                let fecha_registro_str = match doc.get("fecha_registro") {
                    Some(bson_fecha) => {
                        let dt = bson_fecha.as_datetime().ok_or(
                            PostulanteError::PostulanteRepositorioError(
                                RepositorioError::LecturaNoFinalizada,
                            ),
                        )?;
                        {
                            let millis = dt.timestamp_millis();
                            let secs = millis / 1000;
                            let nanos = ((millis % 1000) * 1_000_000) as u32;
                            let naive = chrono::DateTime::from_timestamp(secs, nanos)
                                .unwrap_or_default()
                                .naive_utc();
                            naive.format("%Y-%m-%d %H:%M:%S").to_string()
                        }
                    },
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
                let fecha_registro = FechaRegistro::new(&fecha_registro_str)?;

                Ok(Postulante {
                    id,
                    documento,
                    nombre_completo,
                    fecha_nacimiento,
                    grado_instruccion,
                    genero,
                    password: None,
                    fecha_registro,
                })
            }
            Ok(None) => {
                error!("No postulante found with documento: {}", postulante_id);
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::RegistroNoEncontrado,
                ))
            }
            Err(e) => {
                error!(
                    "Database error while fetching postulante with documento={}, error={}",
                    postulante_id, e
                );
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::LecturaNoFinalizada,
                ))
            }
        }
    }

    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
        let sort = doc! { "fecha_registro": -1 };
        match self.get_collection().find(doc! {}).sort(sort).await {
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
                            let id = match doc.get("_id") {
                                Some(doc_bson) => doc_bson
                                    .as_str()
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
                                    .to_string(),
                                None => {
                                    return Err(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ));
                                }
                            };

                            let fecha_registro_str = match doc.get("fecha_registro") {
                                Some(bson_fecha) => bson_fecha
                                    .as_str()
                                    .ok_or(PostulanteError::PostulanteRepositorioError(
                                        RepositorioError::LecturaNoFinalizada,
                                    ))?
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
                            let fecha_registro = FechaRegistro::new(&fecha_registro_str)?;

                            Ok(Postulante {
                                id,
                                documento,
                                nombre_completo,
                                fecha_nacimiento,
                                grado_instruccion,
                                genero,
                                password: None,
                                fecha_registro,
                            })
                        })() {
                            Ok(postulante) => Some(postulante),
                            Err(e) => {
                                error!(
                                    "Error al convertir documento MongoDB a entidad Postulante: {e}",
                                );
                                None
                            }
                        }
                    })
                    .collect();

                Ok(postulantes)
            }
            Err(e) => {
                error!("Error de base de datos al obtener la lista de postulantes: {e}",);
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::LecturaNoFinalizada,
                ))
            }
        }
    }
}
