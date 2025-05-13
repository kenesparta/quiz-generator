use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct RegistrarPostulanteDTO {
    id: String,
    documento: String,
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    fecha_nacimiento: String,
    grado_instruccion: String,
    genero: String,
}
