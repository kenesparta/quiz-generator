//! Un único mapeo de errores a HTTP.
//!
//! Los handlers devuelven `Result<HttpResponse, ApiError>` y propagan con `?`: cada error del
//! dominio se traduce a un estado HTTP aquí, una sola vez y con `match` exhaustivo (un variante
//! nuevo obliga a decidir su estado). El cuerpo es siempre `{"error": "<mensaje>"}`. En los
//! 5xx el mensaje es genérico y la causa solo va al log.
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use log::{error, warn};
use quizz_auth::universal::domain::error::login_universal::LoginUniversalError;
use quizz_common::domain::value_objects::id::IdError;
use quizz_core::admin::domain::error::admin::{AdminError, RepositorioError as RepoAdmin};
use quizz_core::evaluacion::domain::error::evaluacion::{
    EvaluacionError, RepositorioError as RepoEvaluacion,
};
use quizz_core::examen::domain::error::examen::{ExamenError, RepositorioError as RepoExamen};
use quizz_core::postulante::domain::error::postulante::{
    PostulanteError, RepositorioError as RepoPostulante,
};
use quizz_core::pregunta::domain::error::pregunta::PreguntaError;
use quizz_core::psicologo::domain::error::psicologo::{
    PsicologoError, RepositorioError as RepoPsicologo,
};
use quizz_core::respuesta::domain::error::respuesta::RespuestaError;
use std::fmt;

#[derive(Debug)]
pub enum ApiError {
    /// 400: la petición no es válida (formato, valores fuera de dominio...).
    SolicitudInvalida(String),
    /// 401: faltan credenciales o no son válidas.
    NoAutenticado(String),
    /// 403: autenticado, pero sin permiso para esta acción.
    Prohibido(String),
    /// 404: el recurso no existe (o no es del usuario: no se revela cuál de las dos).
    NoEncontrado(String),
    /// 409: la petición choca con el estado actual del recurso.
    Conflicto(String),
    /// 429: demasiados intentos; se puede reintentar tras `reintentar_en` segundos.
    DemasiadosIntentos { reintentar_en: u64 },
    /// 500: fallo interno; la causa ya se registró al construirlo con [`ApiError::interno`].
    Interno,
    /// 503: una dependencia (base de datos, caché) no está disponible.
    NoDisponible,
}

impl ApiError {
    pub fn solicitud_invalida(mensaje: impl fmt::Display) -> Self {
        Self::SolicitudInvalida(mensaje.to_string())
    }

    pub fn no_encontrado(mensaje: impl fmt::Display) -> Self {
        Self::NoEncontrado(mensaje.to_string())
    }

    pub fn conflicto(mensaje: impl fmt::Display) -> Self {
        Self::Conflicto(mensaje.to_string())
    }

    pub fn prohibido(mensaje: impl fmt::Display) -> Self {
        Self::Prohibido(mensaje.to_string())
    }

    /// Registra la causa en el log y devuelve un 500 sin detalles para el cliente.
    pub fn interno(causa: impl fmt::Display) -> Self {
        error!("error interno: {causa}");
        Self::Interno
    }
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::SolicitudInvalida(m)
            | Self::NoAutenticado(m)
            | Self::Prohibido(m)
            | Self::NoEncontrado(m)
            | Self::Conflicto(m) => f.write_str(m),
            Self::DemasiadosIntentos { .. } => {
                f.write_str("Demasiados intentos, espere antes de volver a intentarlo")
            }
            Self::Interno => f.write_str("Error interno del servidor"),
            Self::NoDisponible => f.write_str("Servicio no disponible, intente más tarde"),
        }
    }
}

impl ResponseError for ApiError {
    fn status_code(&self) -> StatusCode {
        match self {
            Self::SolicitudInvalida(_) => StatusCode::BAD_REQUEST,
            Self::NoAutenticado(_) => StatusCode::UNAUTHORIZED,
            Self::Prohibido(_) => StatusCode::FORBIDDEN,
            Self::NoEncontrado(_) => StatusCode::NOT_FOUND,
            Self::Conflicto(_) => StatusCode::CONFLICT,
            Self::DemasiadosIntentos { .. } => StatusCode::TOO_MANY_REQUESTS,
            Self::Interno => StatusCode::INTERNAL_SERVER_ERROR,
            Self::NoDisponible => StatusCode::SERVICE_UNAVAILABLE,
        }
    }

    fn error_response(&self) -> HttpResponse {
        let mut respuesta = HttpResponse::build(self.status_code());
        if let Self::DemasiadosIntentos { reintentar_en } = self {
            respuesta.insert_header((actix_web::http::header::RETRY_AFTER, *reintentar_en));
        }
        respuesta.json(serde_json::json!({ "error": self.to_string() }))
    }
}

impl From<IdError> for ApiError {
    fn from(e: IdError) -> Self {
        Self::solicitud_invalida(e)
    }
}

impl From<RespuestaError> for ApiError {
    fn from(e: RespuestaError) -> Self {
        use RespuestaError::*;
        match e {
            AsignarIDRespuestaError(_) | CantidadDeRespuestasNoValida | RespuestaNoValida => {
                Self::solicitud_invalida(e)
            }
            EvaluacionRespuestaNotFound
            | PostulanteRespuestaNotFound
            | RespuestaNoEncontrada
            | PreguntaNotFound
            | ExamenNotFound => Self::no_encontrado(e),
            EvaluacionAlreadyAssigned
            | EvaluacionNoPublicada
            | EvaluacionNoEstaEnProceso
            | EvaluacionYaIniciada
            | EvaluacionFinalizada
            | EvaluacionNoFinalizada
            | TransicionNoAplicada => Self::conflicto(e),
            DatabaseError | RepositorioError => Self::interno(e),
        }
    }
}

impl From<EvaluacionError> for ApiError {
    fn from(e: EvaluacionError) -> Self {
        use EvaluacionError::*;
        match e {
            EvaluacionIdInvalido(_)
            | ExamenIdNoValido(_)
            | SinExamenes
            | NombreNoValido
            | DescripcionNoValida => Self::solicitud_invalida(e),
            ExamenNoExiste => Self::no_encontrado(e),
            EvaluacionRepositorioError(RepoEvaluacion::EvaluacionNoExiste) => {
                Self::no_encontrado(e)
            }
            EvaluacionYaFuePublicada
            | EvaluacionSinPreguntas
            | EvaluacionRepositorioError(RepoEvaluacion::RegistroDuplicado) => Self::conflicto(e),
            // Estados guardados que no se pueden interpretar: datos corruptos, no culpa del cliente.
            EvaluacionEstadoGeneralError(_)
            | EvaluacionEstadoError(_)
            | EvaluacionRepositorioError(
                RepoEvaluacion::PersistenciaNoFinalizada | RepoEvaluacion::LecturaNoFinalizada,
            ) => Self::interno(e),
        }
    }
}

impl From<ExamenError> for ApiError {
    fn from(e: ExamenError) -> Self {
        use ExamenError::*;
        match e {
            ExamenIdInvalido(_)
            | TituloInvalido
            | DescripcionInvalida
            | PuntajeIgualQueCero
            | DuracionInvalida
            | PuntosTotalesInvalidos
            | CategoriaInvalida
            | NivelDificultadInvalido
            | SinPreguntas
            | TipoExamenNoValido => Self::solicitud_invalida(e),
            NoEncontrado => Self::no_encontrado(e),
            ExamenRepositorioError(RepoExamen::RegistroDuplicado) => Self::conflicto(e),
            ExamenEstadoGeneralError(_)
            | RepositorioError(_)
            | Desconocido(_)
            | ExamenRepositorioError(
                RepoExamen::PersistenciaNoFinalizada | RepoExamen::LecturaNoFinalizada,
            ) => Self::interno(e),
        }
    }
}

impl From<PreguntaError> for ApiError {
    fn from(e: PreguntaError) -> Self {
        use PreguntaError::*;
        match e {
            AlternativasNoExisten
            | AlternativaNoAjustada
            | PuntajeNoExiste
            | DebeTenerUnaSolaRespuesta
            | PuntajeNoCoincideConAlternativa
            | ImagenNoValida
            | PreguntaErrorExamenID(_)
            | PreguntaAlternativaError(_)
            | PreguntaEtiquetaError(_)
            | PreguntaTipoPreguntaError(_) => Self::solicitud_invalida(e),
            PreguntaRepositorioError(_) => Self::interno(e),
        }
    }
}

impl From<PostulanteError> for ApiError {
    fn from(e: PostulanteError) -> Self {
        use PostulanteError::*;
        match e {
            PostulanteIdError(_)
            | PostulanteDocumentoError(_)
            | PostulanteNombreError(_)
            | PostulanteFechaNacimientoError(_)
            | PostulanteGradoInstruccionError(_)
            | PostulanteGeneroError(_) => Self::solicitud_invalida(e),
            PasswordNoCoincide => Self::NoAutenticado(e.to_string()),
            Cifrado(_) => Self::interno(e),
            PostulanteRepositorioError(RepoPostulante::RegistroDuplicado) => Self::conflicto(e),
            PostulanteRepositorioError(RepoPostulante::RegistroNoEncontrado) => {
                Self::no_encontrado("Postulante no encontrado")
            }
            // Hash o fecha de registro guardados que no se pueden leer: datos corruptos.
            PostulantePasswordError(_)
            | PostulanteFechaRegistroError(_)
            | PostulanteRepositorioError(
                RepoPostulante::PersistenciaNoFinalizada
                | RepoPostulante::LecturaNoFinalizada
                | RepoPostulante::PasswordVacio,
            ) => Self::interno(e),
        }
    }
}

impl From<PsicologoError> for ApiError {
    fn from(e: PsicologoError) -> Self {
        use PsicologoError::*;
        match e {
            PsicologoIdError(_) | NombreNoValido(_) | DocumentoNoValido(_) | EspecialidadVacia
            | ColegiaturaVacia | PasswordNoValido(_) => Self::solicitud_invalida(e),
            PsicologoRepositorioError(RepoPsicologo::RegistroDuplicado) => Self::conflicto(e),
            PsicologoRepositorioError(RepoPsicologo::RegistroNoEncontrado) => {
                Self::no_encontrado("Psicologo no encontrado")
            }
            HashVacio
            | Cifrado(_)
            | PsicologoRepositorioError(
                RepoPsicologo::PersistenciaNoFinalizada
                | RepoPsicologo::LecturaNoFinalizada
                | RepoPsicologo::PasswordVacio,
            ) => Self::interno(e),
        }
    }
}

impl From<AdminError> for ApiError {
    fn from(e: AdminError) -> Self {
        use AdminError::*;
        match e {
            AdminIdError(_) | NombreNoValido(_) | DocumentoNoValido(_) | PasswordNoValido(_) => {
                Self::solicitud_invalida(e)
            }
            AdminRepositorioError(RepoAdmin::RegistroDuplicado) => Self::conflicto(e),
            AdminRepositorioError(RepoAdmin::RegistroNoEncontrado) => {
                Self::no_encontrado("Admin no encontrado")
            }
            HashVacio
            | Cifrado(_)
            | AdminRepositorioError(
                RepoAdmin::PersistenciaNoFinalizada
                | RepoAdmin::LecturaNoFinalizada
                | RepoAdmin::PasswordVacio,
            ) => Self::interno(e),
        }
    }
}

impl From<LoginUniversalError> for ApiError {
    fn from(e: LoginUniversalError) -> Self {
        use LoginUniversalError::*;
        match e {
            // La misma respuesta para ambos casos: no revela qué documentos existen.
            UsuarioNoEncontrado | PasswordIncorrecto => {
                warn!("login rechazado: {e}");
                Self::NoAutenticado("Documento o password incorrectos".to_string())
            }
            ErrorGenericoCache => {
                error!("login: {e}");
                Self::NoDisponible
            }
            JWTErrorAlGenerar | RepositorioError | Cifrado(_) => Self::interno(e),
        }
    }
}

/// Errores de los extractores de actix (JSON, query y ruta mal formados) con el mismo formato
/// `{"error": ...}` y su estado original (400, 413, 415...).
pub fn error_de_extraccion<E: ResponseError + 'static>(
    err: E,
    _req: &actix_web::HttpRequest,
) -> actix_web::Error {
    let respuesta = HttpResponse::build(err.status_code())
        .json(serde_json::json!({ "error": err.to_string() }));
    actix_web::error::InternalError::from_response(err, respuesta).into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cada_categoria_tiene_su_estado() {
        let casos = [
            (ApiError::solicitud_invalida("x"), StatusCode::BAD_REQUEST),
            (
                ApiError::NoAutenticado("x".into()),
                StatusCode::UNAUTHORIZED,
            ),
            (ApiError::prohibido("x"), StatusCode::FORBIDDEN),
            (ApiError::no_encontrado("x"), StatusCode::NOT_FOUND),
            (ApiError::conflicto("x"), StatusCode::CONFLICT),
            (ApiError::Interno, StatusCode::INTERNAL_SERVER_ERROR),
            (ApiError::NoDisponible, StatusCode::SERVICE_UNAVAILABLE),
        ];
        for (error, estado) in casos {
            assert_eq!(error.status_code(), estado, "{error:?}");
        }
    }

    #[test]
    fn un_error_interno_no_revela_la_causa() {
        let error = ApiError::interno("conexión rechazada por mongo:27017");
        assert_eq!(error.to_string(), "Error interno del servidor");
    }

    #[test]
    fn errores_del_dominio_con_estado_correcto() {
        assert_eq!(
            ApiError::from(RespuestaError::EvaluacionAlreadyAssigned).status_code(),
            StatusCode::CONFLICT
        );
        assert_eq!(
            ApiError::from(RespuestaError::RespuestaNoEncontrada).status_code(),
            StatusCode::NOT_FOUND
        );
        assert_eq!(
            ApiError::from(EvaluacionError::NombreNoValido).status_code(),
            StatusCode::BAD_REQUEST
        );
        assert_eq!(
            ApiError::from(LoginUniversalError::PasswordIncorrecto).status_code(),
            StatusCode::UNAUTHORIZED
        );
        assert_eq!(
            ApiError::from(LoginUniversalError::UsuarioNoEncontrado).to_string(),
            ApiError::from(LoginUniversalError::PasswordIncorrecto).to_string()
        );
    }
}
