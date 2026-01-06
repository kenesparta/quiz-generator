use evaluacion_common::{Entity, Id, SimpleName};

use crate::pregunta::value_object::{
    AlternativasMultiples, Etiqueta, ImagenRef, PuntajeAlternativaUnica,
};

/// Pregunta de alternativa única.
///
/// El postulante debe elegir UNA sola alternativa de entre varias opciones.
/// Solo una alternativa es correcta y otorga puntaje.
///
/// # Ejemplo de uso
///
/// ```json
/// {
///   "contenido": "Lo opuesto al ODIO es:",
///   "alternativas": {
///     "A": "Enemigo",
///     "B": "Temor",
///     "C": "Amor",
///     "D": "Amigo",
///     "E": "Alegría"
///   },
///   "puntaje": { "C": 1 }
/// }
/// ```
#[derive(Debug, Clone)]
pub struct PreguntaAlternativaUnica {
    id: Id,
    contenido: SimpleName,
    imagen: Option<ImagenRef>,
    etiqueta: Etiqueta,
    alternativas: AlternativasMultiples,
    puntaje: PuntajeAlternativaUnica,
}

impl PreguntaAlternativaUnica {
    pub fn new(
        contenido: SimpleName,
        imagen: Option<ImagenRef>,
        etiqueta: Etiqueta,
        alternativas: AlternativasMultiples,
        puntaje: PuntajeAlternativaUnica,
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
        puntaje: PuntajeAlternativaUnica,
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
    pub fn puntaje(&self) -> &PuntajeAlternativaUnica {
        &self.puntaje
    }
}

impl Entity for PreguntaAlternativaUnica {
    fn id(&self) -> Id {
        self.id
    }
}

impl PartialEq for PreguntaAlternativaUnica {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for PreguntaAlternativaUnica {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::value_object::{Alternativa, AlternativaClave, Puntaje};

    fn crear_alternativas_ejemplo() -> AlternativasMultiples {
        let items = vec![
            Alternativa::new(AlternativaClave::A, "Opción A".to_string()),
            Alternativa::new(AlternativaClave::B, "Opción B".to_string()),
            Alternativa::new(AlternativaClave::C, "Opción C".to_string()),
        ];
        AlternativasMultiples::new(items).unwrap()
    }

    #[test]
    fn test_crear_pregunta() {
        let contenido = SimpleName::new("¿Cuál es la capital de Francia?".to_string()).unwrap();
        let alternativas = crear_alternativas_ejemplo();
        let puntaje = PuntajeAlternativaUnica::new(AlternativaClave::A, Puntaje::uno()).unwrap();

        let pregunta = PreguntaAlternativaUnica::new(
            contenido.clone(),
            None,
            Etiqueta::No,
            alternativas,
            puntaje,
        );

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
        assert!(pregunta.imagen().is_none());
        assert_eq!(pregunta.alternativas().len(), 3);
    }

    #[test]
    fn test_entidad_igualdad_por_id() {
        let contenido = SimpleName::new("Test".to_string()).unwrap();
        let alternativas = crear_alternativas_ejemplo();
        let puntaje = PuntajeAlternativaUnica::new(AlternativaClave::A, Puntaje::uno()).unwrap();

        let p1 = PreguntaAlternativaUnica::new(
            contenido.clone(),
            None,
            Etiqueta::No,
            alternativas.clone(),
            puntaje.clone(),
        );
        let p2 =
            PreguntaAlternativaUnica::new(contenido, None, Etiqueta::No, alternativas, puntaje);

        // Diferentes IDs = diferentes entidades
        assert_ne!(p1, p2);
    }
}
