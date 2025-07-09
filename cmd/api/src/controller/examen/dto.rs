use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrarExamenDTO {
    pub titulo: String,
    pub descripcion: String,
    pub instrucciones: String,
}
