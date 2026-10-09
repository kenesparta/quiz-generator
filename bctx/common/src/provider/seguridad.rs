use async_trait::async_trait;
use thiserror::Error;

/// Fallo técnico al calcular o verificar un hash. Una contraseña que no coincide no es un
/// error: [`Cifrador::verificar`] devuelve `Ok(false)`.
#[derive(Debug, Error)]
pub enum CifradoError {
    #[error("no se pudo calcular el hash de la contraseña")]
    Hash,

    #[error("el hash guardado no es válido")]
    HashNoValido,

    #[error("la tarea de cifrado no terminó")]
    Tarea,
}

/// Hash y verificación de contraseñas. La implementación elige el algoritmo y se ocupa de no
/// bloquear el ejecutor async (el hash es lento a propósito).
#[async_trait]
pub trait Cifrador: Send + Sync {
    async fn cifrar(&self, password: String) -> Result<String, CifradoError>;

    /// `Ok(true)` si `password` corresponde a `hash`.
    async fn verificar(&self, password: String, hash: String) -> Result<bool, CifradoError>;

    /// Hace el mismo trabajo que [`Cifrador::verificar`] sin un hash real. Sirve para que
    /// rechazar un usuario inexistente tarde lo mismo que rechazar una contraseña incorrecta:
    /// si no, el tiempo de respuesta revela qué usuarios existen.
    async fn simular_verificacion(&self, password: String);
}
