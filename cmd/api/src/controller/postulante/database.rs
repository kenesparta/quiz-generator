use actix_web::web;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteEscritura;

pub struct PostulantePostgres {
    pool: web::Data<sqlx::PgPool>,
}

impl PostulantePostgres {
    pub fn new(pool: web::Data<sqlx::PgPool>) -> Self {
        PostulantePostgres { pool }
    }
}

// sqlx::Error
impl RepositorioPostulanteEscritura<PostulanteError> for PostulantePostgres {
    fn registrar_postulante(&self, postulante: Postulante) -> Result<(), PostulanteError> {
        let pool = self.pool.as_ref();

        let result = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(async {
                // Create the SQL query
                sqlx::query(
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
                .bind(postulante.id.to_string())
                .bind(postulante.documento.to_string())
                .bind(postulante.nombre_completo.nombre())
                .bind(postulante.nombre_completo.primer_apellido())
                .bind(postulante.nombre_completo.segundo_apellido())
                .bind(postulante.fecha_nacimiento.to_string())
                .bind(postulante.grado_instruccion.to_string())
                .bind(postulante.genero.to_string())
                .bind(postulante.password.to_string())
                .execute(pool)
                .await
            })
        });

        match result {
            Ok(_) => Ok(()),
            Err(e) => {
                log::error!("Database error: {}", e);
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }
}
