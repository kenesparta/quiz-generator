use crate::controller::auth::mongo::constantes::POSTULANTE_AUTH_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::postulante::domain::postulante::PostulanteLogin;
use quizz_auth::postulante::provider::repositorio::RepositorioPostulanteLoginLectura;

pub struct PostulanteLoginMongo {
    client: web::Data<mongodb::Client>,
}

impl PostulanteLoginMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for PostulanteLoginMongo {
    fn get_collection_name(&self) -> &str {
        POSTULANTE_AUTH_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioPostulanteLoginLectura<PostulanteLoginError> for PostulanteLoginMongo {
    async fn obtener_postulante_por_documento(
        &self,
        documento: String,
    ) -> Result<PostulanteLogin, PostulanteLoginError> {
        let filter = doc! { "documento": documento.clone() };

        match self.get_collection().find_one(filter).await {
            Ok(Some(doc)) => {
                let id = match doc.get("_id") {
                    Some(doc_bson) => doc_bson.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteLoginError::RepositorioError);
                    }
                };

                let documento_db = match doc.get("documento") {
                    Some(doc_bson) => doc_bson.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteLoginError::RepositorioError);
                    }
                };

                let password_db = match doc.get("password") {
                    Some(pass_bson) => pass_bson.as_str().unwrap_or_default().to_string(),
                    None => {
                        return Err(PostulanteLoginError::RepositorioError);
                    }
                };

                Ok(PostulanteLogin {
                    id,
                    usuario: documento_db,
                    password: password_db,
                })
            }

            Ok(None) => Err(PostulanteLoginError::PostulanteNoEncontrado),
            Err(_) => Err(PostulanteLoginError::RepositorioError),
        }
    }
}
