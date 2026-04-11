use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DocumentoLoginRequestDTO {
    pub documento: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub token: String,
    pub expires_in: u64,
    pub rol: String,
}
