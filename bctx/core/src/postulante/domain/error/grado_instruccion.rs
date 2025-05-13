use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum GradoInstruccionError {
    #[error("Grado de instrucción no válido")]
    NoValido,
}
