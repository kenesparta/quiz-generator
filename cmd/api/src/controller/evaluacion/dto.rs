use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrarEvaluacionDTO {
    pub nombre: String,
    pub descripcion: String,
    pub estado: String,
}

#[derive(Deserialize)]
pub struct AgregarExamenesDTO {
    pub examenes: Vec<String>,
}
