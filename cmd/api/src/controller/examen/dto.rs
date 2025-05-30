use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegistrarExamenDTO {
    pub titulo: String,
    pub descripcion: String,
    pub puntaje_maximo: u32,
}
