use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Password vacio")]
    Vacio,

    #[error("Hash no valido")]
    HashNoValido,

    #[error("error en el cifrado del password")]
    CifradoNoValido,

    #[error("error al momento de la verificacion")]
    NoVerificado,
}
