use crate::evaluacion::value_object::id::EvaluacionID;
use crate::postulante::domain::value_object::id::PostulanteID;
use crate::respuesta::domain::entity::pregunta::PreguntaACorregir;
use crate::respuesta::domain::entity::respuesta::{Estado, Respuesta, RespuestaEvaluacion};
use crate::respuesta::domain::entity::revision::RevisionRealizada;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use crate::respuesta::domain::value_object::id::RespuestaID;
use chrono::{DateTime, FixedOffset};

pub trait RepositorioRespuestaEscritura: Send + Sync {
    /// Crea la hoja `id` con una copia de la evaluación publicada para el postulante.
    ///
    /// Falla con `EvaluacionRespuestaNotFound`/`PostulanteRespuestaNotFound` si alguno no
    /// existe, con `EvaluacionNoPublicada` si la evaluación es un borrador y con
    /// `EvaluacionAlreadyAssigned` si el postulante ya la tiene (lo garantiza un índice único,
    /// también con peticiones simultáneas).
    fn asignar_evaluacion(
        &self,
        id: &RespuestaID,
        evaluacion_id: EvaluacionID,
        postulante_id: PostulanteID,
    ) -> impl Future<Output = Result<(), RespuestaError>> + Send;

    /// Guarda la contestación (respuestas y puntos) solo si la hoja es del postulante y sigue
    /// en proceso, en una única operación atómica. Devuelve `false` si no se escribió.
    fn responder_evaluacion(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> impl Future<Output = Result<bool, RespuestaError>> + Send;

    /// Estado de la hoja y pregunta a corregir. Falla con `RespuestaNoEncontrada` si la hoja no
    /// existe o es de otro postulante, y con `ExamenNotFound`/`PreguntaNotFound` si la
    /// pregunta no está en la hoja.
    fn obtener_pregunta(
        &self,
        respuesta_evaluacion: &RespuestaEvaluacion,
    ) -> impl Future<Output = Result<PreguntaACorregir, RespuestaError>> + Send;
}

/// Estado de una hoja de respuestas y sus transiciones.
pub trait RepositorioEstadoRespuesta: Send + Sync {
    /// Estado de la hoja si pertenece a `postulante_id`; `None` si no existe o es de otro.
    fn obtener_estado(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
    ) -> impl Future<Output = Result<Option<Estado>, RespuestaError>> + Send;

    /// Pasa la hoja de `desde` a `hacia` en una única operación atómica (compare-and-set) y
    /// registra `fecha` como inicio (al pasar a `EnProceso`) o fin (al pasar a `Finalizado`).
    /// Devuelve `false` si no existe, es de otro postulante o ya no estaba en `desde`.
    fn transicionar(
        &self,
        id: &RespuestaID,
        postulante_id: &PostulanteID,
        desde: Estado,
        hacia: Estado,
        fecha: DateTime<FixedOffset>,
    ) -> impl Future<Output = Result<bool, RespuestaError>> + Send;
}

pub trait RepositorioRespuestaLectura: Send + Sync {
    /// La hoja `respuesta_id`. Con `postulante_id` solo la devuelve si es suya (lectura del
    /// postulante); sin él, la de cualquiera (lectura del personal). Falla con
    /// `RespuestaNoEncontrada` si no hay coincidencia.
    fn obtener(
        &self,
        respuesta_id: &RespuestaID,
        postulante_id: Option<&PostulanteID>,
    ) -> impl Future<Output = Result<Respuesta, RespuestaError>> + Send;
}

pub trait RepositorioRespuestaRevision: Send + Sync {
    /// Resumen de las hojas en `estado`, de la más reciente a la más antigua. Solo los campos
    /// del listado: la hoja completa incluye la evaluación con sus imágenes.
    fn obtener_respuesta_revision(
        &self,
        estado: Estado,
    ) -> impl Future<
        Output = Result<
            Vec<crate::respuesta::use_case::respuesta_revision::OutputData>,
            RespuestaError,
        >,
    > + Send;
}

pub trait RepositorioRealizarRevision: Send + Sync {
    /// Guarda la revisión en una sola operación atómica, solo si la hoja está finalizada.
    ///
    /// Falla con `RespuestaNoEncontrada` si la hoja no existe o es de otra evaluación, con
    /// `EvaluacionNoFinalizada` si todavía se puede contestar y con `ExamenNotFound` si algún
    /// examen no está en la hoja.
    fn realizar_revision(
        &self,
        revision: &RevisionRealizada,
    ) -> impl Future<Output = Result<(), RespuestaError>> + Send;
}

pub trait RepositorioObtenerRevisionPorId: Send + Sync {
    fn obtener_revision_por_id(
        &self,
        revision_id: String,
    ) -> impl Future<Output = Result<Respuesta, RespuestaError>> + Send;
}

pub trait RepositorioListaRespuestaPostulante: Send + Sync {
    fn obtener_respuestas_por_postulante(
        &self,
        postulante_id: crate::postulante::domain::value_object::id::PostulanteID,
    ) -> impl Future<
        Output = Result<
            Vec<crate::respuesta::use_case::lista_respuesta_postulante::OutputData>,
            RespuestaError,
        >,
    > + Send;
}

pub trait RepositorioListarAsignaciones: Send + Sync {
    fn listar(
        &self,
        postulante_id: Option<String>,
        evaluacion_id: Option<String>,
    ) -> impl Future<
        Output = Result<
            Vec<crate::respuesta::use_case::listar_asignaciones::OutputData>,
            RespuestaError,
        >,
    > + Send;
}
