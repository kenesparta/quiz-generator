use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrarEvaluacionDTO {
    pub titulo: String,
    pub descripcion: String,
}

#[derive(Deserialize)]
pub struct AgregarExamenesDTO {
    pub examenes: Vec<String>,
}
