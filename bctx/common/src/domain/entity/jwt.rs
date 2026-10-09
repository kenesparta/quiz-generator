use std::fmt;

/// Token de sesión emitido en el login. `value` es el bearer token: `Debug` lo omite.
#[derive(Clone)]
pub struct JwtObject {
    pub key: String,
    pub value: String,
    pub expiration: u64,
    pub rol: Option<String>,
}

impl fmt::Debug for JwtObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("JwtObject")
            .field("key", &self.key)
            .field("expiration", &self.expiration)
            .field("rol", &self.rol)
            .finish_non_exhaustive()
    }
}
