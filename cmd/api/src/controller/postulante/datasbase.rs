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
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), sqlx::Error> {
        sqlx::query!(
            r#"
            INSERT INTO postulante (
                id,
                documento,
                nombre,
                apellido_paterno,
                apellido_materno,
                fecha_nacimiento,
                grado_instruccion,
                genero,
                created_at,
                updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), NOW())
            "#,
            postulante.id,
            postulante.documento,
            postulante.nombre,
            postulante.apellido_paterno,
            postulante.apellido_materno,
            postulante.fecha_nacimiento,
            postulante.grado_instruccion,
            postulante.genero,
        ).execute(self.pool.get_ref());
        Ok(())
    }
}