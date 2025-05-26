use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegistrarExamenDTO {
    pub id: String,
    pub titulo: String,
    pub descripcion: String,
    pub activo: bool,
}
