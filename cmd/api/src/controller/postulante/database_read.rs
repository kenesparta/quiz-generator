use crate::controller::postulante::dto::PostulanteDataBaseDTO;
use actix_web::web;
use async_trait::async_trait;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::documento::Documento;
use quizz_core::postulante::domain::value_object::genero::Genero;
use quizz_core::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::domain::value_object::nombre::Nombre;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteLectura;
use std::str::FromStr;
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
        let pool = self.pool.as_ref();
        let doc_string = documento.to_string();

        match sqlx::query_as::<_, PostulanteDataBaseDTO>(
            r#"
        SELECT
            id,
            documento,
            nombre,
            primer_apellido,
            segundo_apellido,
            fecha_nacimiento,
            grado_instruccion,
            genero
        FROM postulante
        WHERE documento = $1
        "#,
        )
        .bind(&doc_string)
        .fetch_optional(pool)
        .await
        {
            Ok(Some(dto)) => {
                let id = PostulanteID::new(&dto.id.to_string())?;
                let documento = Documento::new(&dto.documento.to_string())?;
                let nombre_completo =
                    Nombre::new(dto.nombre, dto.primer_apellido, dto.segundo_apellido)?;
                let fecha_nacimiento = FechaNacimiento::new(&dto.fecha_nacimiento)?;
                let grado_instruccion = GradoInstruccion::from_str(&dto.grado_instruccion)?;
                let genero = Genero::from_str(&dto.genero)?;

                Ok(Postulante {
                    id,
                    documento,
                    nombre_completo,
                    fecha_nacimiento,
                    grado_instruccion,
                    genero,
                    password: None,
                })
            }
            Ok(None) => {
                error!("No postulante found with documento: {}", doc_string);
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::RegistroNoEncontrado,
                ))
            }
            Err(e) => {
                error!(
                    "Database error while fetching postulante with documento={}, error={}",
                    doc_string, e
                );
                Err(PostulanteError::PostulanteRepositorioError(
                    RepositorioError::PersistenciaNoFinalizada,
                ))
            }
        }
    }

    async fn obtener_postulante_por_id(
        &self,
        _postulante_id: PostulanteID,
    ) -> Result<Postulante, PostulanteError> {
        todo!()
    }

    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
        todo!()
    }
}
