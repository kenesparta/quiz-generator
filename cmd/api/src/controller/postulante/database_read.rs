use actix_web::web;
use async_trait::async_trait;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::documento::Documento;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteLectura;
use tracing::log::error;

pub struct PostulanteReadPostgres {
    pool: web::Data<sqlx::PgPool>,
}

impl PostulanteReadPostgres {
    pub fn new(pool: web::Data<sqlx::PgPool>) -> Self {
        PostulanteReadPostgres { pool }
    }
}

#[async_trait]
impl RepositorioPostulanteLectura<PostulanteError> for PostulanteReadPostgres {
    async fn obtener_postulante_por_documento(
        &self,
        documento: Documento,
    ) -> Result<Postulante, PostulanteError> {
        todo!()
    }

    async fn obtener_postulante_por_id(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<Postulante, PostulanteError> {
        todo!()
    }

    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
        todo!()
    }
}
