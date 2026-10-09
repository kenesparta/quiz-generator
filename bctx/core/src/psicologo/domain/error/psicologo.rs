use quizz_common::domain::value_objects::id::IdError;
use quizz_common::domain::value_objects::password_plano::PasswordPlanoError;
use quizz_common::provider::seguridad::CifradoError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PsicologoError {
    #[error("Psicologo ID no valido: {0}")]
    PsicologoIdError(#[from] IdError),

    #[error("Nombre no valido: {0}")]
    NombreNoValido(String),

    #[error("Documento no valido: {0}")]
    DocumentoNoValido(String),

    #[error("Especialidad vacia")]
    EspecialidadVacia,

    #[error("Colegiatura vacia")]
    ColegiaturaVacia,

    #[error("Password no valido: {0}")]
    PasswordNoValido(#[from] PasswordPlanoError),

    /// Invariante interna: la entidad siempre recibe un hash, nunca vacío.
    #[error("El hash del password esta vacio")]
    HashVacio,

    #[error("Error al cifrar el password: {0}")]
    Cifrado(#[from] CifradoError),

    #[error("Error al manipular la base de datos: {0}")]
    PsicologoRepositorioError(#[from] RepositorioError),
}

#[derive(Error, Debug)]
pub enum RepositorioError {
    #[error("Error al persistir")]
    PersistenciaNoFinalizada,

    #[error("Error al leer")]
    LecturaNoFinalizada,

    #[error("El password esta vacio antes de ejecutar la persistencia")]
    PasswordVacio,

    #[error("registro no encontrado")]
    RegistroNoEncontrado,

    #[error("Ya existe un registro con ese id o documento")]
    RegistroDuplicado,
}
