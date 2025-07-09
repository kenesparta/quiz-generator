use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct RegistrarPostulanteDTO {
    pub documento: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}

#[derive(Deserialize)]
pub struct PostulanteDocumentoQuery {
    pub documento: Option<String>,
}

#[derive(Serialize)]
pub struct PostulanteResponseDTO {
    pub id: String,
    pub documento: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub nombre_completo: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
    #[serde(rename = "_links")]
    pub links_: Links,
}

#[derive(Serialize)]
pub struct Links {
    pub self_: String,
    pub update: String,
    pub delete: String,
    pub exams: String,
    pub results: String,
}
