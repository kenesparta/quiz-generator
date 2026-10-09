use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::pregunta::PreguntaACorregir;
use crate::respuesta::domain::entity::respuesta::{
    Estado, Respuesta, RespuestaEvaluacion, Revision,
};
use crate::respuesta::domain::entity::revision::ExamenRevision;
use crate::respuesta::domain::value_object::id::RespuestaID;
use async_trait::async_trait;
use chrono::{DateTime, FixedOffset};

#[async_trait]
pub trait RepositorioRespuestaEscritura<Error>: Send + Sync {
    /// Crea la hoja `id` con una copia de la evaluación publicada para el postulante.
    ///
    /// Falla con `EvaluacionRespuestaNotFound`/`PostulanteRespuestaNotFound` si alguno no
    /// existe, con `EvaluacionNoPublicada` si la evaluación es un borrador y con
    /// `EvaluacionAlreadyAssigned` si el postulante ya la tiene (lo garantiza un índice único,
    /// también con peticiones simultáneas).
    async fn asignar_evaluacion(
        &self,
        id: &RespuestaID,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> Result<(), Error>;

    /// Guarda la contestación (respuestas y puntos) solo si la hoja es del postulante y sigue
    /// en proceso, en una única operación atómica. Devuelve `false` si no se escribió.
    async fn responder_evaluacion(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<bool, Error>;

    /// Estado de la hoja y pregunta a corregir. Falla con `RespuestaNoEncontrada` si la hoja no
    /// existe o es de otro postulante, y con `ExamenNotFound`/`PreguntaNotFound` si la
    /// pregunta no está en la hoja.
    async fn obtener_pregunta(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> Result<PreguntaACorregir, Error>;
}

/// Estado de una hoja de respuestas y sus transiciones.
#[async_trait]
pub trait RepositorioEstadoRespuesta<Error>: Send + Sync {
    /// Estado de la hoja si pertenece a `postulante_id`; `None` si no existe o es de otro.
    async fn obtener_estado(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
    ) -> Result<Option<Estado>, Error>;

    /// Pasa la hoja de `desde` a `hacia` en una única operación atómica (compare-and-set) y
    /// registra `fecha` como inicio (al pasar a `EnProceso`) o fin (al pasar a `Finalizado`).
    /// Devuelve `false` si no existe, es de otro postulante o ya no estaba en `desde`.
    async fn transicionar(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
        desde: Estado,
        hacia: Estado,
        fecha: DateTime<FixedOffset>,
    ) -> Result<bool, Error>;
}

#[async_trait]
pub trait RepositorioRespuestaLectura<Error>: Send + Sync {
    /// La hoja `respuesta_id`. Con `postulante_id` solo la devuelve si es suya (lectura del
    /// postulante); sin él, la de cualquiera (lectura del personal). Falla con
    /// `RespuestaNoEncontrada` si no hay coincidencia.
    async fn obtener(
        &self,
        respuesta_id: &RespuestaID,
        postulante_id: Option<&PostulanteID>,
    ) -> Result<Respuesta, Error>;
}

#[async_trait]
pub trait RespositorioRespuestaRevision<Error>: Send + Sync {
    async fn obtener_respuesta_revision(&self, estado: Estado) -> Result<Vec<Respuesta>, Error>;
}

#[async_trait]
pub trait RespositorioRealizarRevision<Error>: Send + Sync {
    async fn realizar_revision(
        &self,
        revision_id: String,
        evaluacion_id: String,
        examenes: Vec<ExamenRevision>,
        estado: Revision,
        resultado: String, // Deberia ser enum
    ) -> Result<(), Error>;
}

#[async_trait]
pub trait RepositorioObtenerRevisionPorId<Error>: Send + Sync {
    async fn obtener_revision_por_id(&self, revision_id: String) -> Result<Respuesta, Error>;
}

#[async_trait]
pub trait RepositorioListaRespuestaPostulante<Error>: Send + Sync {
    async fn obtener_respuestas_por_postulante(
        &self,
        postulante_id: crate::postulante::domain::value_object::id::PostulanteID,
    ) -> Result<Vec<crate::respuesta::use_case::lista_respuesta_postulante::OutputData>, Error>;
}

#[async_trait]
pub trait RepositorioListarAsignaciones<Error>: Send + Sync {
    async fn listar(
        &self,
        postulante_id: Option<String>,
        evaluacion_id: Option<String>,
    ) -> Result<Vec<crate::respuesta::use_case::listar_asignaciones::OutputData>, Error>;
}
