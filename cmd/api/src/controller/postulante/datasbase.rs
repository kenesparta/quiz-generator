use actix_web::web;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;

pub struct PostulantePostgres {
    pool: web::Data<sqlx::PgPool>,   
}

impl PostulantePostgres {
    pub fn new(pool: web::Data<sqlx::PgPool>) -> Self {
        PostulantePostgres { pool }
    }
}

impl RepositorioPostulanteEscritura<sqlx::Error> for PostulantePostgres {
    fn registrar_postulante(&self, postulante: Postulante) -> Result<(), sqlx::Error> {
        Ok(())
    }
}