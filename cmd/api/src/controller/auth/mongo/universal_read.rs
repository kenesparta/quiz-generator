use crate::controller::auth::mongo::constantes::{
    ADMIN_AUTH_COLLECTION_NAME, POSTULANTE_AUTH_COLLECTION_NAME, PSICOLOGO_AUTH_COLLECTION_NAME,
};
use actix_web::web;
use async_trait::async_trait;
use mongodb::bson::{Document, doc};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_auth::universal::domain::usuario_login::UsuarioLogin;
use quizz_auth::universal::provider::repositorio::RepositorioLoginUniversalLectura;

const DATABASE_NAME: &str = "quizz";

pub struct LoginUniversalMongo {
    client: web::Data<mongodb::Client>,
}

impl LoginUniversalMongo {
    pub fn new(client: web::Data<mongodb::Client>) -> Self {
        Self { client }
    }

    async fn buscar_en_coleccion(
        &self,
        collection_name: &str,
        documento: &str,
        rol: Rol,
    ) -> Result<Option<UsuarioLogin>, LoginUniversalError> {
        let collection = self
            .client
            .database(DATABASE_NAME)
            .collection::<Document>(collection_name);

        let filter = doc! { "documento": documento };

        match collection.find_one(filter).await {
            Ok(Some(doc)) => {
                let id = doc
                    .get("_id")
                    .and_then(|v| v.as_str())
                    .ok_or(LoginUniversalError::RepositorioError)?
                    .to_string();

                let password = doc
                    .get("password")
                    .and_then(|v| v.as_str())
                    .ok_or(LoginUniversalError::RepositorioError)?
                    .to_string();

                Ok(Some(UsuarioLogin {
                    id,
                    password,
                    rol: rol.to_string(),
                }))
            }
            Ok(None) => Ok(None),
            Err(_) => Err(LoginUniversalError::RepositorioError),
        }
    }
}

#[async_trait]
impl RepositorioLoginUniversalLectura<LoginUniversalError> for LoginUniversalMongo {
    async fn buscar_por_documento(
        &self,
        documento: String,
    ) -> Result<UsuarioLogin, LoginUniversalError> {
        // Buscar en orden: admin -> psicologo -> postulante
        if let Some(usuario) = self
            .buscar_en_coleccion(ADMIN_AUTH_COLLECTION_NAME, &documento, Rol::Admin)
            .await?
        {
            return Ok(usuario);
        }

        if let Some(usuario) = self
            .buscar_en_coleccion(PSICOLOGO_AUTH_COLLECTION_NAME, &documento, Rol::Psicologo)
            .await?
        {
            return Ok(usuario);
        }

        if let Some(usuario) = self
            .buscar_en_coleccion(POSTULANTE_AUTH_COLLECTION_NAME, &documento, Rol::Postulante)
            .await?
        {
            return Ok(usuario);
        }

        Err(LoginUniversalError::UsuarioNoEncontrado)
    }
}
