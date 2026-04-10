use crate::controller::auth::mongo::constantes::ADMIN_AUTH_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_auth::admin::domain::admin::AdminLogin;
use quizz_auth::admin::domain::error::admin::AdminLoginError;
use quizz_auth::admin::provider::repositorio::RepositorioAdminLoginLectura;

pub struct AdminLoginMongo {
    client: web::Data<mongodb::Client>,
}

impl AdminLoginMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }
}

impl MongoRepository for AdminLoginMongo {
    fn get_collection_name(&self) -> &str {
        ADMIN_AUTH_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioAdminLoginLectura<AdminLoginError> for AdminLoginMongo {
    async fn obtener_admin_por_documento(
        &self,
        documento: String,
    ) -> Result<AdminLogin, AdminLoginError> {
        let filter = doc! { "documento": documento };

        match self.get_collection().find_one(filter).await {
            Ok(Some(doc)) => {
                let id = doc
                    .get("_id")
                    .and_then(|v| v.as_str())
                    .ok_or(AdminLoginError::RepositorioError)?
                    .to_string();

                let documento_db = doc
                    .get("documento")
                    .and_then(|v| v.as_str())
                    .ok_or(AdminLoginError::RepositorioError)?
                    .to_string();

                let password_db = doc
                    .get("password")
                    .and_then(|v| v.as_str())
                    .ok_or(AdminLoginError::RepositorioError)?
                    .to_string();

                Ok(AdminLogin {
                    id,
                    documento: documento_db,
                    password: password_db,
                })
            }
            Ok(None) => Err(AdminLoginError::AdminNoEncontrado),
            Err(_) => Err(AdminLoginError::RepositorioError),
        }
    }
}
