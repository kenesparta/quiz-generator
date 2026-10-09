use crate::examen::domain::error::examen::ExamenError;
use crate::examen::provider::repositorio::RepositorioExamenListar;

#[derive(Debug, Clone)]
pub struct OutputData {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
    pub estado: String,
    pub cantidad_preguntas: usize,
}

pub struct ListarExamenes<R> {
    repositorio: R,
}

impl<R: RepositorioExamenListar> ListarExamenes<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self) -> Result<Vec<OutputData>, ExamenError> {
        self.repositorio.listar_examenes().await
    }
}
