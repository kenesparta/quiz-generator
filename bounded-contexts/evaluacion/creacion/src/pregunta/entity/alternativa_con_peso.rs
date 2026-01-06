use common::{Entity, Id, SimpleName};

use crate::pregunta::value_object::{AlternativasMultiples, Etiqueta, ImagenRef, PuntajeConPeso};

/// Pregunta con peso por alternativa.
///
/// El postulante elige UNA alternativa, pero cada alternativa tiene
/// un puntaje diferente (escala Likert, por ejemplo).
///
/// # Ejemplo de uso
///
/// ```json
/// {
///   "contenido": "¿Qué tan satisfecho está con el servicio?",
///   "alternativas": {
///     "A": "Muy insatisfecho",
///     "B": "Insatisfecho",
///     "C": "Neutral",
///     "D": "Satisfecho",
///     "E": "Muy satisfecho"
///   },
///   "puntaje": { "A": 0, "B": 1, "C": 2, "D": 3, "E": 4 }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PreguntaAlternativaConPeso {
    id: Id,
    contenido: SimpleName,
    imagen: Option<ImagenRef>,
    etiqueta: Etiqueta,
    alternativas: AlternativasMultiples,
    puntaje: PuntajeConPeso,
}

impl PreguntaAlternativaConPeso {
    pub fn new(
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        alternativas: AlternativasMultiples,
        puntaje: PuntajeConPeso,
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
        alternativas: AlternativasMultiples,
        puntaje: PuntajeConPeso,
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
    pub fn alternativas(&self) -> &AlternativasMultiples {
        &self.alternativas
    }

    #[must_use]
    pub fn puntaje(&self) -> &PuntajeConPeso {
        &self.puntaje
    }
}

impl Entity for PreguntaAlternativaConPeso {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for PreguntaAlternativaConPeso {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PreguntaAlternativaConPeso {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::value_object::{Alternativa, AlternativaClave};

    fn crear_alternativas_likert() -> AlternativasMultiples {
        let items = vec![
            Alternativa::new(AlternativaClave::A, "Muy insatisfecho".to_string()),
            Alternativa::new(AlternativaClave::B, "Insatisfecho".to_string()),
            Alternativa::new(AlternativaClave::C, "Neutral".to_string()),
            Alternativa::new(AlternativaClave::D, "Satisfecho".to_string()),
            Alternativa::new(AlternativaClave::E, "Muy satisfecho".to_string()),
        ];
        AlternativasMultiples::new(items).unwrap()
    }

    #[test]
    fn test_crear_pregunta_likert() {
        let contenido =
            SimpleName::new("¿Qué tan satisfecho está con el servicio?".to_string()).unwrap();
        let alternativas = crear_alternativas_likert();
        let puntaje = PuntajeConPeso::from_vec(vec![
            (AlternativaClave::A, 0.0),
            (AlternativaClave::B, 1.0),
            (AlternativaClave::C, 2.0),
            (AlternativaClave::D, 3.0),
            (AlternativaClave::E, 4.0),
        ])
        .unwrap();

        let pregunta = PreguntaAlternativaConPeso::new(
            contenido.clone(),
            None,
            Etiqueta::Extrovertido,
            alternativas,
            puntaje,
        );

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
        assert_eq!(pregunta.alternativas().len(), 5);
    }
}
