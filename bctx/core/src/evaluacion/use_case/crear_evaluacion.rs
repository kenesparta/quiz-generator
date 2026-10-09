use crate::evaluacion::domain::entity::evaluacion::Evaluacion;
use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioEvaluacionEscritura;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
}

pub struct CrearEvaluacion<R> {
    repositorio: R,
}

impl<R: RepositorioEvaluacionEscritura> CrearEvaluacion<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        self.repositorio
            .guardar_evaluacion(Evaluacion::new(in_.id, in_.titulo, in_.descripcion)?)
            .await?;
        Ok(())
    }
}
