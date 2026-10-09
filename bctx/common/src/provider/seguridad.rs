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
/// bloquear el ejecutor async (el hash es lento a propósito). Los casos de uso lo reciben como
/// genérico; el adaptador lo implementa con `async fn`.
pub trait Cifrador: Send + Sync {
    fn cifrar(&self, password: String)
    -> impl Future<Output = Result<String, CifradoError>> + Send;

    /// `Ok(true)` si `password` corresponde a `hash`.
    fn verificar(
        &self,
        password: String,
        hash: String,
    ) -> impl Future<Output = Result<bool, CifradoError>> + Send;

    /// Hace el mismo trabajo que [`Cifrador::verificar`] sin un hash real. Sirve para que
    /// rechazar un usuario inexistente tarde lo mismo que rechazar una contraseña incorrecta:
    /// si no, el tiempo de respuesta revela qué usuarios existen.
    fn simular_verificacion(&self, password: String) -> impl Future<Output = ()> + Send;
}
