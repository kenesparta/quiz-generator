use crate::examen::domain::entity::examen::Examen;
use crate::examen::domain::error::examen::ExamenError;
use crate::examen::provider::repositorio::RepositorioExamenEscritura;

#[derive(Debug, Clone)]
pub struct InputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
}

pub struct CrearExamen<R> {
    repositorio: R,
}

impl<R: RepositorioExamenEscritura> CrearExamen<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), ExamenError> {
        let examen = Examen::new(
            in_.id.to_string(),
            in_.titulo,
            in_.descripcion,
            in_.instrucciones,
        )?;
        self.repositorio.guardar_examen(examen).await?;
        Ok(())
    }
}
