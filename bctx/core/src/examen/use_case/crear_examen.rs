use crate::examen::domain::entity::examen::Examen;
use crate::examen::domain::error::examen::ExamenError;
use crate::examen::provider::repositorio::RepositorioExamenEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub activo: bool,
}

#[derive(Debug, Clone)]
pub struct OutputData {}

pub struct CrearExamen<RepoErr> {
    repositorio: Box<dyn RepositorioExamenEscritura<RepoErr>>,
}

impl<RepoErr> CrearExamen<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioExamenEscritura<RepoErr>>) -> CrearExamen<RepoErr> {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, ExamenError> for CrearExamen<RepoErr>
where
    ExamenError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, ExamenError> {
        let examen = Examen::new(in_.id.to_string(), in_.titulo, in_.descripcion, in_.activo)?;
        self.repositorio.guardar_examen(examen).await?;
        Ok(OutputData {})
    }
}
