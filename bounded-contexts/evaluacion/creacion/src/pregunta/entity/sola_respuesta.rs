use common::{Entity, Id, SimpleName};

use crate::pregunta::value_object::{Etiqueta, ImagenRef, PuntajeSolaRespuesta};

/// Pregunta de sola respuesta.
///
/// El postulante escribe una respuesta corta que se compara con la respuesta correcta.
/// Si coincide (ignorando mayúsculas/minúsculas y espacios), se otorga el puntaje.
///
/// # Ejemplo de uso
///
/// ```json
/// {
///   "contenido": "¿Cuál es la capital de Francia?",
///   "respuesta_correcta": "París",
///   "puntaje": 1
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PreguntaSolaRespuesta {
    id: Id,
    contenido: SimpleName,
    imagen: Option<ImagenRef>,
    etiqueta: Etiqueta,
    puntaje: PuntajeSolaRespuesta,
}

impl PreguntaSolaRespuesta {
    pub fn new(
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        puntaje: PuntajeSolaRespuesta,
    ) -> Self {
        Self {
            id: Id::new(),
            contenido,
            imagen,
            etiqueta,
            puntaje,
        }
    }

    pub fn with_id(
        id: Id,
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        puntaje: PuntajeSolaRespuesta,
    ) -> Self {
        Self {
            id,
            contenido,
            imagen,
            etiqueta,
            puntaje,
        }
    }

    #[must_use]
    pub fn contenido(&self) -> &SimpleName {
        &self.contenido
    }

    #[must_use]
    pub fn imagen(&self) -> Option<&ImagenRef> {
        self.imagen.as_ref()
    }

    #[must_use]
    pub fn etiqueta(&self) -> &Etiqueta {
        &self.etiqueta
    }

    #[must_use]
    pub fn puntaje(&self) -> &PuntajeSolaRespuesta {
        &self.puntaje
    }
}

impl Entity for PreguntaSolaRespuesta {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for PreguntaSolaRespuesta {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PreguntaSolaRespuesta {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::value_object::Puntaje;

    #[test]
    fn test_crear_pregunta_sola_respuesta() {
        let contenido = SimpleName::new("¿Cuál es la capital de Francia?".to_string()).unwrap();
        let puntaje = PuntajeSolaRespuesta::new("París".to_string(), Puntaje::uno());

        let pregunta =
            PreguntaSolaRespuesta::new(contenido.clone(), None, Etiqueta::No, puntaje.clone());

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
        assert_eq!(pregunta.puntaje().respuesta_correcta(), "París");
        assert_eq!(pregunta.puntaje().puntaje().valor(), 1.0);
    }
}
