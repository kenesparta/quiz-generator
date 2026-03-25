use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct RegistrarUsuarioDTO {
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

#[derive(Serialize)]
pub struct UsuarioResponseDTO {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub rol: String,
}
