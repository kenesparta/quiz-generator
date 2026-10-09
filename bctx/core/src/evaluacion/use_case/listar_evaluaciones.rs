use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioEvaluacionListar;

#[derive(Debug, Clone)]
pub struct OutputData {
    pub id: String,
    pub nombre: String,
    pub descripcion: String,
    pub estado: String,
    pub esta_activo: String,
    pub cantidad_examenes: usize,
}

pub struct ListarEvaluaciones<R> {
    repositorio: R,
}

impl<R: RepositorioEvaluacionListar> ListarEvaluaciones<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self) -> Result<Vec<OutputData>, EvaluacionError> {
        let evaluaciones = self.repositorio.listar_evaluaciones().await?;
        Ok(evaluaciones)
    }
}
