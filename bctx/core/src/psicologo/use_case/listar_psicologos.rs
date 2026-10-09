use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::provider::repositorio::RepositorioPsicologoListar;

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

pub struct ListarPsicologos<R> {
    repositorio: R,
}

impl<R: RepositorioPsicologoListar> ListarPsicologos<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self) -> Result<Vec<OutputData>, PsicologoError> {
        let psicologos = self.repositorio.listar_psicologos().await?;
        Ok(psicologos)
    }
}
