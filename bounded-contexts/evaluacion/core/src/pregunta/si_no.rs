use evaluacion_common::{Entity, Id, SimpleName};

use crate::pregunta::value_object::{AlternativasSiNo, Etiqueta, ImagenRef, PuntajeSiNo};

/// Pregunta de Sí o No.
///
/// El postulante elige entre SI o NO.
/// Cada opción puede tener un puntaje diferente.
///
/// # Ejemplo de uso
///
/// ```json
/// {
///   "contenido": "¿Le gusta trabajar en equipo?",
///   "alternativas": { "SI": "Sí", "NO": "No" },
///   "puntaje": { "SI": 1, "NO": 0 }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PreguntaSiNo {
    id: Id,
    contenido: SimpleName,
    imagen: Option<ImagenRef>,
    etiqueta: Etiqueta,
    alternativas: AlternativasSiNo,
    puntaje: PuntajeSiNo,
}

impl PreguntaSiNo {
    pub fn new(
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        alternativas: AlternativasSiNo,
        puntaje: PuntajeSiNo,
    ) -> Self {
        Self {
            id: Id::new(),
            contenido,
            imagen,
            etiqueta,
            alternativas,
            puntaje,
        }
    }

    pub fn with_id(
        id: Id,
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        alternativas: AlternativasSiNo,
        puntaje: PuntajeSiNo,
    ) -> Self {
        Self {
            id,
            contenido,
            imagen,
            etiqueta,
            alternativas,
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
    pub fn alternativas(&self) -> &AlternativasSiNo {
        &self.alternativas
    }

    #[must_use]
    pub fn puntaje(&self) -> &PuntajeSiNo {
        &self.puntaje
    }
}

impl Entity for PreguntaSiNo {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for PreguntaSiNo {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PreguntaSiNo {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::value_object::Puntaje;

    #[test]
    fn test_crear_pregunta_si_no() {
        let contenido = SimpleName::new("¿Le gusta trabajar en equipo?".to_string()).unwrap();
        let alternativas = AlternativasSiNo::default_texts();
        let puntaje = PuntajeSiNo::solo_si(Puntaje::uno());

        let pregunta = PreguntaSiNo::new(
            contenido.clone(),
            None,
            Etiqueta::Extrovertido,
            alternativas,
            puntaje.clone(),
        );

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
    }

    #[test]
    fn test_pregunta_si_no_con_textos_personalizados() {
        let contenido = SimpleName::new("¿Es verdadero o falso?".to_string()).unwrap();
        let alternativas = AlternativasSiNo::new("Verdadero".to_string(), "Falso".to_string());
        let puntaje = PuntajeSiNo::new(Puntaje::new(2.0).unwrap(), Puntaje::cero());

        let pregunta =
            PreguntaSiNo::new(contenido, None, Etiqueta::Honestidad, alternativas, puntaje);

        assert_eq!(pregunta.alternativas().texto_si(), "Verdadero");
        assert_eq!(pregunta.alternativas().texto_no(), "Falso");
    }
}
