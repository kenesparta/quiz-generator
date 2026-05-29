use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::provider::repositorio::RepositorioPsicologoListar;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData;

#[derive(Debug, Clone)]
pub struct OutputData {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub especialidad: String,
    pub colegiatura: String,
}

pub struct ListarPsicologos<RepoErr> {
    repositorio: Box<dyn RepositorioPsicologoListar<RepoErr>>,
}

impl<RepoErr> ListarPsicologos<RepoErr> {
    pub fn new(repositorio: Box<dyn RepositorioPsicologoListar<RepoErr>>) -> Self {
        Self { repositorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, Vec<OutputData>, PsicologoError> for ListarPsicologos<RepoErr>
where
    PsicologoError: From<RepoErr>,
{
    async fn ejecutar(&self, _input: InputData) -> Result<Vec<OutputData>, PsicologoError> {
        let psicologos = self.repositorio.listar_psicologos().await?;
        Ok(psicologos)
    }
}
