use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::provider::repositorio::RepositorioListarAsignaciones;

pub struct InputData {
    pub postulante_id: Option<String>,
    pub evaluacion_id: Option<String>,
}

pub struct OutputData {
    pub respuesta_id: String,
    pub estado: String,
    pub fecha_tiempo_inicio: String,
    pub fecha_tiempo_fin: String,
    pub evaluacion_id: String,
    pub evaluacion_nombre: String,
    pub evaluacion_descripcion: String,
    pub postulante_id: String,
    pub postulante_documento: String,
    pub postulante_nombre: String,
    pub postulante_primer_apellido: String,
    pub postulante_segundo_apellido: String,
}

pub struct ListarAsignaciones<R> {
    repo: R,
}

impl<R: RepositorioListarAsignaciones> ListarAsignaciones<R> {
    pub fn new(repo: R) -> Self {
        Self { repo }
    }

    /// Los filtros se validan como ids: uno mal formado es un error, no un listado vacío, y
    /// uno en mayúsculas encuentra los registros guardados en minúsculas.
    pub async fn ejecutar(&self, input: InputData) -> Result<Vec<OutputData>, RespuestaError> {
        let postulante_id = input
            .postulante_id
            .as_deref()
            .map(PostulanteID::new)
            .transpose()?;
        let evaluacion_id = input
            .evaluacion_id
            .as_deref()
            .map(EvaluacionID::new)
            .transpose()?;
        self.repo.listar(postulante_id, evaluacion_id).await
    }
}
