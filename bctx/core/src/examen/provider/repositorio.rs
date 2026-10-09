//! Puertos de exámenes. Los casos de uso los reciben como genéricos (despacho estático), no
//! como `dyn`: por eso son `async` nativos y declaran que su futuro es `Send`. El adaptador
//! los implementa con `async fn`.

use crate::examen::domain::entity::examen::Examen;
use crate::examen::domain::error::examen::ExamenError;
use crate::examen::use_case::listar_examenes::OutputData;

pub trait RepositorioExamenEscritura: Send + Sync {
    fn guardar_examen(
        &self,
        examen: Examen,
    ) -> impl Future<Output = Result<(), ExamenError>> + Send;
}

pub trait RepositorioExamenLectura: Send + Sync {
    fn obtener_examen(&self, id: &str) -> impl Future<Output = Result<Examen, ExamenError>> + Send;
}

pub trait RepositorioExamenListar: Send + Sync {
    fn listar_examenes(&self) -> impl Future<Output = Result<Vec<OutputData>, ExamenError>> + Send;
}
