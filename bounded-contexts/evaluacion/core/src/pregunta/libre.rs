use evaluacion_common::{Entity, Id, SimpleName};

use crate::pregunta::value_object::{Etiqueta, ImagenRef};

/// Pregunta de respuesta libre.
///
/// El postulante escribe una respuesta de texto libre.
/// No tiene alternativas predefinidas ni puntaje automático.
///
/// # Ejemplo de uso
///
/// ```json
/// {
///   "contenido": "Describa su experiencia laboral más relevante"
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PreguntaLibre {
    id: Id,
    contenido: SimpleName,
    imagen: Option<ImagenRef>,
    etiqueta: Etiqueta,
}

impl PreguntaLibre {
    pub fn new(contenido: SimpleName, imagen: Option<ImagenRef>, etiqueta: Etiqueta) -> Self {
        Self {
            id: Id::new(),
            contenido,
            imagen,
            etiqueta,
        }
    }

    pub fn with_id(
        id: Id,
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
    ) -> Self {
        Self {
            id,
            contenido,
            imagen,
            etiqueta,
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
}

impl Entity for PreguntaLibre {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for PreguntaLibre {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PreguntaLibre {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_pregunta_libre() {
        let contenido =
            SimpleName::new("Describa su experiencia laboral más relevante".to_string()).unwrap();
        let pregunta = PreguntaLibre::new(contenido.clone(), None, Etiqueta::No);

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
        assert!(pregunta.imagen().is_none());
    }

    #[test]
    fn test_pregunta_libre_con_imagen() {
        let contenido = SimpleName::new("Describa lo que ve en la imagen".to_string()).unwrap();
        let imagen = ImagenRef::new("https://example.com/test.png".to_string()).unwrap();
        let pregunta = PreguntaLibre::new(contenido, Some(imagen), Etiqueta::No);

        assert!(pregunta.imagen().is_some());
        assert_eq!(
            pregunta.imagen().unwrap().valor(),
            "https://example.com/test.png"
        );
    }
}
