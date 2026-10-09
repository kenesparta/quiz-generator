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

    pub async fn ejecutar(&self, input: InputData) -> Result<Vec<OutputData>, RespuestaError> {
        let asignaciones = self
            .repo
            .listar(input.postulante_id, input.evaluacion_id)
            .await?;
        Ok(asignaciones)
    }
}
