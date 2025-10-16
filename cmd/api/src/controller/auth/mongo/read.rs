use crate::controller::auth::mongo::constantes::POSTULANTE_AUTH_COLLECTION_NAME;
use crate::controller::mongo_repository::MongoRepository;
use actix_web::web;
use async_trait::async_trait;
use quizz_auth::postulante::domain::error::postulante::PostulanteLoginError;
use quizz_auth::postulante::provider::repositorio::RepositorioPostulanteLoginEscritura;

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
impl RepositorioPostulanteLoginEscritura<PostulanteLoginError> for PostulanteLoginMongo {
    async fn login(&self, usuario: String, password: String) -> Result<(), PostulanteLoginError> {
        Ok(())
    }
}
