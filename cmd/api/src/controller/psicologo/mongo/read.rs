use crate::controller::mongo_repository::MongoRepository;
use crate::controller::psicologo::mongo::write::PsicologoMongo;
use mongodb::bson::doc;
use quizz_core::psicologo::domain::error::psicologo::{PsicologoError, RepositorioError};
use quizz_core::psicologo::provider::repositorio::{
    PsicologoInfo, RepositorioPsicologoLectura, RepositorioPsicologoListar,
};
use quizz_core::psicologo::use_case::listar_psicologos::OutputData;
use tracing::error;

impl RepositorioPsicologoLectura for PsicologoMongo {
    async fn obtener_psicologo_por_id(&self, id: String) -> Result<PsicologoInfo, PsicologoError> {
        let filter = doc! { "_id": &id };

        let doc = self.get_collection().find_one(filter).await.map_err(|e| {
            error!("Error finding psicologo by id {}: {}", id, e);
            PsicologoError::PsicologoRepositorioError(RepositorioError::LecturaNoFinalizada)
        })?;

        match doc {
            Some(doc) => {
                let nombre = doc.get_str("nombre").unwrap_or_default().to_string();
                let primer_apellido = doc
                    .get_str("primer_apellido")
                    .unwrap_or_default()
                    .to_string();
                let segundo_apellido = doc
                    .get_str("segundo_apellido")
                    .unwrap_or_default()
                    .to_string();
                let colegiatura = doc.get_str("colegiatura").unwrap_or_default().to_string();

                Ok(PsicologoInfo {
                    nombre,
                    primer_apellido,
                    segundo_apellido,
                    colegiatura,
                })
            }
            None => Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroNoEncontrado,
            )),
        }
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
