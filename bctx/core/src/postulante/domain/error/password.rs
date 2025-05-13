use thiserror::Error;

#[derive(Error, Debug)]
pub enum PasswordError {
    #[error("Password vacio")]
    Vacio,

    #[error("Hash no valido")]
    HashNoValido,
}
