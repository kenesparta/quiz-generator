use crate::controller::auth::mongo::constantes::PSICOLOGO_AUTH_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_auth::psicologo::domain::error::psicologo::PsicologoLoginError;
use quizz_auth::psicologo::domain::psicologo::PsicologoLogin;
use quizz_auth::psicologo::provider::repositorio::RepositorioPsicologoLoginLectura;

pub struct PsicologoLoginMongo {
    client: web::Data<mongodb::Client>,
}

impl PsicologoLoginMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for PsicologoLoginMongo {
    fn get_collection_name(&self) -> &str {
        PSICOLOGO_AUTH_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioPsicologoLoginLectura<PsicologoLoginError> for PsicologoLoginMongo {
    async fn obtener_psicologo_por_documento(
        &self,
        documento: String,
    ) -> Result<PsicologoLogin, PsicologoLoginError> {
        let filter = doc! { "documento": documento };

        match self.get_collection().find_one(filter).await {
            Ok(Some(doc)) => {
                let id = doc
                    .get("_id")
                    .and_then(|v| v.as_str())
                    .ok_or(PsicologoLoginError::RepositorioError)?
                    .to_string();

                let documento_db = doc
                    .get("documento")
                    .and_then(|v| v.as_str())
                    .ok_or(PsicologoLoginError::RepositorioError)?
                    .to_string();

                let password_db = doc
                    .get("password")
                    .and_then(|v| v.as_str())
                    .ok_or(PsicologoLoginError::RepositorioError)?
                    .to_string();

                Ok(PsicologoLogin {
                    id,
                    documento: documento_db,
                    password: password_db,
                })
            }
            Ok(None) => Err(PsicologoLoginError::PsicologoNoEncontrado),
            Err(_) => Err(PsicologoLoginError::RepositorioError),
        }
    }
}
