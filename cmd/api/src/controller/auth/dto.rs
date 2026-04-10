use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct PostulanteLoginRequestDTO {
    pub user_name: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct PostulanteLoginResponseDTO {
    pub token: String,
    pub expires_in: u64,
}

#[derive(Serialize, Deserialize)]
pub struct DocumentoLoginRequestDTO {
    pub documento: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
pub struct LoginResponseDTO {
    pub token: String,
    pub expires_in: u64,
}
