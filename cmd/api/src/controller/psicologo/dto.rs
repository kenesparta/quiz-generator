use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct RegistrarPsicologoDTO {
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub especialidad: String,
    pub password: String,
}

#[derive(Serialize)]
#[allow(dead_code)]
pub struct PsicologoResponseDTO {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub especialidad: String,
}
