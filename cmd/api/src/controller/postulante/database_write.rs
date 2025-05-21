use actix_web::web;
use async_trait::async_trait;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use tracing::log::error;

pub struct PostulantePostgres {
    pool: web::Data<sqlx::PgPool>,
}

impl PostulantePostgres {
    pub fn new(pool: web::Data<sqlx::PgPool>) -> Self {
        PostulantePostgres { pool }
    }
}

#[async_trait]
impl RepositorioPostulanteEscritura<PostulanteError> for PostulantePostgres {
    async fn registrar_postulante(&self, postulante: Postulante) -> Result<(), PostulanteError> {
        let pool = self.pool.as_ref();
        let password = postulante
            .password
            .ok_or(PostulanteError::PostulanteRepositorioError(
                RepositorioError::PasswordVacio,
            ))?
            .value();

        match sqlx::query(
            "INSERT INTO postulante (
                id, 
                documento, 
                nombre,
                primer_apellido,
                segundo_apellido,
                fecha_nacimiento, 
                grado_instruccion, 
                genero, 
                password
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)",
        )
        .bind(postulante.id.value().uuid())
        .bind(postulante.documento.to_string())
        .bind(postulante.nombre_completo.nombre())
        .bind(postulante.nombre_completo.primer_apellido())
        .bind(postulante.nombre_completo.segundo_apellido())
        .bind(postulante.fecha_nacimiento.to_string())
        .bind(postulante.grado_instruccion.to_string())
        .bind(postulante.genero.to_string())
        .bind(password)
        .execute(pool)
        .await
        {
            Ok(_) => Ok(()),
            Err(e) => {
                error!(
                    "Database error while registering postulante: id={}, documento={}, error={}",
                    postulante.id, postulante.documento, e
                );

                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn actualizar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<(), PostulanteError> {
        todo!()
    }

    async fn eliminar_postulante(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<(), PostulanteError> {
        todo!()
    }
}
