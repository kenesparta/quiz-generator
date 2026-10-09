use crate::controller::hateoas::{Links, enlaces};
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
    pub id: Option<String>,
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
    pub fecha_registro: String,
    #[serde(rename = "_links")]
    pub links: Links,
}

/// El documento (dato personal) no va en los enlaces: terminaría en logs de proxies.
pub fn build_postulante_links(postulante_id: &str) -> Links {
    enlaces::postulante(postulante_id)
}
