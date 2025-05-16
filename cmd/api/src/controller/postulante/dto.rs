use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegistrarPostulanteDTO {
    pub documento: String,
    pub nombre: String,
    pub apellido_paterno: String,
    pub apellido_materno: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}
