use crate::controller::mongo_repository::MongoRepository;
use crate::controller::usuario::mongo::constantes::USUARIO_COLLECTION_NAME;
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::doc;
use quizz_auth::usuario::domain::entity::usuario::Usuario;
use quizz_auth::usuario::domain::error::usuario::UsuarioError;
use quizz_auth::usuario::provider::repositorio::RepositorioUsuarioEscritura;
use tracing::log::error;

pub struct UsuarioMongo {
    client: web::Data<mongodb::Client>,
}

impl UsuarioMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        UsuarioMongo { client }
    }
}

impl MongoRepository for UsuarioMongo {
    fn get_collection_name(&self) -> &str {
        USUARIO_COLLECTION_NAME
    }

    fn get_client(&self) -> &web::Data<mongodb::Client> {
        &self.client
    }
}

#[async_trait]
impl RepositorioUsuarioEscritura<UsuarioError> for UsuarioMongo {
    async fn registrar_usuario(&self, usuario: Usuario) -> Result<(), UsuarioError> {
        let documento = doc! {
            "_id": &usuario.id,
            "nombre": &usuario.nombre,
            "email": &usuario.email,
            "password": &usuario.password,
            "rol": usuario.rol.to_string(),
        };

        match self.get_collection().insert_one(documento).await {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Database error while registering usuario: id={}, email={}, error={}",
                    usuario.id, usuario.email, e
                );
                Err(UsuarioError::RepositorioError)
            }
        }
    }
}
