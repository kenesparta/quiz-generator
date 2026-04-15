use crate::controller::hateoas::{Link, Links};
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

pub fn build_postulante_links(postulante_id: &str, documento: &str) -> Links {
    let mut links = Links::new();
    links.insert(
        "self".into(),
        Link::get(format!("/postulantes?id={}", postulante_id)),
    );
    links.insert(
        "self_by_documento".into(),
        Link::get(format!("/postulantes?documento={}", documento)),
    );
    links.insert(
        "update".into(),
        Link::put(format!("/postulantes/{}", postulante_id)),
    );
    links.insert(
        "delete".into(),
        Link::delete(format!("/postulantes/{}", postulante_id)),
    );
    links.insert(
        "respuestas".into(),
        Link::get(format!("/respuestas?postulante_id={}", postulante_id)),
    );
    links
}
