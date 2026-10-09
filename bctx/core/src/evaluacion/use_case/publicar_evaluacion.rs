use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::provider::repositorio::RepositorioPublicarEvaluacion;
use crate::evaluacion::value_object::id::EvaluacionID;

#[derive(Debug, Clone)]
pub struct InputData {
    pub evaluacion_id: String,
}

pub struct PublicarEvaluacion<R> {
    repositorio: R,
}

impl<R: RepositorioPublicarEvaluacion> PublicarEvaluacion<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), EvaluacionError> {
        let mut evaluacion = self
            .repositorio
            .obtener_evaluacion(EvaluacionID::new(in_.evaluacion_id.as_str())?)
            .await?;
        evaluacion.publicar()?;
        self.repositorio.publicar_evaluacion(evaluacion).await?;

        Ok(())
    }
}
