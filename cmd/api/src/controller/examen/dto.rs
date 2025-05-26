use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegistrarExamenDTO {
    pub titulo: String,
    pub descripcion: String,
}
