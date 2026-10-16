use crate::mongo::psicologo::write::PsicologoMongo;
use crate::mongo::repositorio::{CAMPO_ELIMINADO, MongoRepository};
use mongodb::bson::{self, doc};
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::domain::value_object::id::PsicologoID;
use quizz_core::psicologo::provider::repositorio::RepositorioPsicologoListar;
use quizz_core::psicologo::use_case::listar_psicologos::OutputData;
use quizz_core::respuesta::domain::entity::revision::Firma;
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use quizz_core::respuesta::provider::repositorio::RepositorioFirma;
use serde::Deserialize;
use tracing::error;

/// Los campos del psicólogo con los que firma una revisión.
#[derive(Deserialize)]
struct DatosFirma {
    nombre: String,
    primer_apellido: String,
    #[serde(default)]
    segundo_apellido: String,
    colegiatura: String,
}

impl RepositorioFirma for PsicologoMongo {
    async fn obtener_firma(&self, id: PsicologoID) -> Result<Option<Firma>, RespuestaError> {
        // Uno marcado para eliminarse ya no firma, igual que ya no inicia sesión.
        let documento = self
            .get_collection()
            .find_one(doc! { "_id": id.to_string(), CAMPO_ELIMINADO: { "$ne": true } })
            .projection(
                doc! { "nombre": 1, "primer_apellido": 1, "segundo_apellido": 1, "colegiatura": 1 },
            )
            .await
            .map_err(|e| {
                error!("mongo, leer la firma del psicologo {id}: {e}");
                RespuestaError::DatabaseError
            })?;
        let Some(documento) = documento else {
            return Ok(None);
        };

        let datos: DatosFirma = bson::from_document(documento).map_err(|e| {
            error!("psicologo {id}: datos de la firma no validos: {e}");
            RespuestaError::DatabaseError
        })?;
        Ok(Some(Firma::new(
            &datos.nombre,
            &datos.primer_apellido,
            &datos.segundo_apellido,
            &datos.colegiatura,
        )))
    }
}

impl RepositorioPsicologoListar for PsicologoMongo {
    async fn listar_psicologos(&self) -> Result<Vec<OutputData>, PsicologoError> {
        // Sin el hash de la contraseña: el listado no lo necesita.
        let mut cursor = self
            .get_collection()
            .find(doc! {})
            .projection(doc! { "password": 0 })
            .await
            .map_err(|e| {
                error!("Database error while listing psicologos: {}", e);
                PsicologoError::PsicologoRepositorioError(RepositorioError::LecturaNoFinalizada)
            })?;

        let mut psicologos = Vec::new();

        while cursor.advance().await.map_err(|e| {
            error!("Error advancing cursor while listing psicologos: {}", e);
            PsicologoError::PsicologoRepositorioError(RepositorioError::LecturaNoFinalizada)
        })? {
            let documento = cursor.deserialize_current().map_err(|e| {
                error!("Error deserializing psicologo document: {}", e);
                PsicologoError::PsicologoRepositorioError(RepositorioError::LecturaNoFinalizada)
            })?;

            let id = documento.get_str("_id").unwrap_or_default().to_string();
            let nombre = documento.get_str("nombre").unwrap_or_default().to_string();
            let primer_apellido = documento
                .get_str("primer_apellido")
                .unwrap_or_default()
                .to_string();
            let segundo_apellido = documento
                .get_str("segundo_apellido")
                .unwrap_or_default()
                .to_string();
            let documento_dni = documento
                .get_str("documento")
                .unwrap_or_default()
                .to_string();
            let especialidad = documento
                .get_str("especialidad")
                .unwrap_or_default()
                .to_string();
            let colegiatura = documento
                .get_str("colegiatura")
                .unwrap_or_default()
                .to_string();

            psicologos.push(OutputData {
                id,
                nombre,
                primer_apellido,
                segundo_apellido,
                documento: documento_dni,
                especialidad,
                colegiatura,
            });
        }

        Ok(psicologos)
    }
}
