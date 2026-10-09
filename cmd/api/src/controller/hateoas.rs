use serde::Serialize;
use std::collections::BTreeMap;

#[derive(Serialize, Clone, Debug)]
pub struct Link {
    pub href: String,
    pub method: &'static str,
}

impl Link {
    pub fn get(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "GET",
        }
    }

    pub fn post(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "POST",
        }
    }

    pub fn patch(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "PATCH",
        }
    }

    pub fn put(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "PUT",
        }
    }

    pub fn delete(href: impl Into<String>) -> Self {
        Self {
            href: href.into(),
            method: "DELETE",
        }
    }
}

/// Enlaces de una respuesta. Ordenados por nombre: el orden no cambia entre procesos.
pub type Links = BTreeMap<String, Link>;

#[derive(Serialize, Debug)]
pub struct ListResponse<T: Serialize> {
    #[serde(rename = "_links")]
    pub links: Links,
    pub items: Vec<T>,
}

/// Los enlaces de cada recurso, en un solo lugar. Solo se anuncian rutas y métodos que existen
/// (lo comprueba un test contra la tabla de rutas real) y, cuando depende del estado, solo las
/// transiciones válidas.
pub mod enlaces {
    use super::{Link, Links};

    /// Un examen: se le pueden agregar preguntas.
    pub fn examen(id: &str) -> Links {
        Links::from([(
            "agregar_preguntas".to_string(),
            Link::put(format!("/examenes/{id}")),
        )])
    }

    /// Una evaluación: en borrador admite exámenes y publicarse; publicada, asignarse.
    pub fn evaluacion(id: &str, publicada: bool) -> Links {
        if publicada {
            Links::from([(
                "asignar".to_string(),
                Link::post(format!("/evaluaciones/{id}/respuestas")),
            )])
        } else {
            Links::from([
                (
                    "asociar_examenes".to_string(),
                    Link::put(format!("/evaluaciones/{id}")),
                ),
                (
                    "publicar".to_string(),
                    Link::patch(format!("/evaluaciones/{id}")),
                ),
            ])
        }
    }

    /// Un postulante: consultarlo, actualizarlo (por documento, en el cuerpo) y sus hojas.
    pub fn postulante(id: &str) -> Links {
        Links::from([
            (
                "self".to_string(),
                Link::get(format!("/postulantes?id={id}")),
            ),
            ("update".to_string(), Link::put("/postulantes")),
            (
                "respuestas".to_string(),
                Link::get(format!("/respuestas?postulante_id={id}")),
            ),
        ])
    }

    /// Un psicólogo: no hay un endpoint para leerlo, solo para eliminarlo.
    pub fn psicologo(id: &str) -> Links {
        Links::from([(
            "eliminar".to_string(),
            Link::delete(format!("/psicologos/{id}")),
        )])
    }

    /// Una asignación (hoja de respuestas de un postulante) en el listado del personal.
    pub fn asignacion(respuesta_id: &str, postulante_id: &str) -> Links {
        Links::from([
            (
                "self".to_string(),
                Link::get(format!("/respuestas/{respuesta_id}")),
            ),
            (
                "postulante".to_string(),
                Link::get(format!("/postulantes?id={postulante_id}")),
            ),
        ])
    }

    /// Una hoja finalizada en el listado de revisiones.
    pub fn revision(respuesta_id: &str, postulante_id: &str) -> Links {
        Links::from([
            (
                "self".to_string(),
                Link::get(format!("/revisiones/{respuesta_id}")),
            ),
            (
                "revisar".to_string(),
                Link::post(format!("/revisiones/{respuesta_id}")),
            ),
            (
                "respuesta".to_string(),
                Link::get(format!("/respuestas/{respuesta_id}")),
            ),
            (
                "postulante".to_string(),
                Link::get(format!("/postulantes?id={postulante_id}")),
            ),
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::controller::respuesta::dto::{build_pregunta_links, build_respuesta_links};
    use crate::prueba_http::{ID, app, estado, token};
    use actix_web::http::{Method, StatusCode};
    use quizz_auth::autorizacion::domain::value_object::rol::Rol;
    use quizz_core::respuesta::domain::entity::respuesta::Estado;

    /// Todos los enlaces que puede anunciar la API, con ids de ejemplo.
    fn todos_los_enlaces() -> Vec<Links> {
        let mut todos = vec![
            enlaces::examen(ID),
            enlaces::evaluacion(ID, false),
            enlaces::evaluacion(ID, true),
            enlaces::postulante(ID),
            enlaces::psicologo(ID),
            enlaces::asignacion(ID, ID),
            enlaces::revision(ID, ID),
        ];
        for estado in [Estado::Creado, Estado::EnProceso, Estado::Finalizado] {
            for rol in [Rol::Postulante, Rol::Psicologo, Rol::Admin] {
                todos.push(build_respuesta_links(ID, estado, rol));
                todos.push(build_pregunta_links(ID, ID, ID, estado, rol));
            }
        }
        todos
    }

    /// Un enlace anunciado debe corresponder a una ruta y un método reales: con un token de
    /// admin, una ruta inexistente responde 404 y un método no admitido 405.
    #[actix_web::test]
    async fn cada_enlace_anunciado_existe_en_la_tabla_de_rutas() {
        let app = app().await;
        let admin = token(Rol::Admin);
        for links in todos_los_enlaces() {
            for (nombre, link) in links {
                let metodo = Method::from_bytes(link.method.as_bytes()).unwrap();
                let estado = estado(&app, metodo, &link.href, Some(&admin)).await;
                assert!(
                    estado != StatusCode::NOT_FOUND && estado != StatusCode::METHOD_NOT_ALLOWED,
                    "{nombre}: {} {} -> {estado}",
                    link.method,
                    link.href
                );
            }
        }
    }

    #[test]
    fn los_enlaces_se_serializan_en_orden() {
        let json = serde_json::to_string(&enlaces::postulante(ID)).unwrap();
        let posiciones: Vec<usize> = ["respuestas", "self", "update"]
            .iter()
            .map(|nombre| json.find(nombre).unwrap())
            .collect();
        assert!(posiciones.windows(2).all(|par| par[0] < par[1]), "{json}");
    }
}
