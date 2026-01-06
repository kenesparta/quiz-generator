use evaluacion_common::{Entity, Id, SimpleName};

use crate::pregunta::alternativa_con_peso::PreguntaAlternativaConPeso;
use crate::pregunta::alternativa_unica::PreguntaAlternativaUnica;
use crate::pregunta::libre::PreguntaLibre;
use crate::pregunta::si_no::PreguntaSiNo;
use crate::pregunta::sola_respuesta::PreguntaSolaRespuesta;
use crate::pregunta::value_object::{Etiqueta, ImagenRef};

/// Sum Type que representa todos los tipos de pregunta posibles.
///
/// Este enum garantiza que cada tipo de pregunta tenga exactamente
/// los campos que necesita, haciendo imposible crear estados inválidos.
///
/// # Ventajas sobre dynamic dispatch
///
/// - **Static dispatch**: No hay costo de indirección en runtime
/// - **Exhaustive matching**: El compilador verifica que manejamos todos los casos
/// - **Type safety**: Imposible mezclar campos de diferentes tipos
/// - **Invariantes por construcción**: Cada variante tiene sus propias reglas
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pregunta {
    AlternativaUnica(PreguntaAlternativaUnica),
    AlternativaConPeso(PreguntaAlternativaConPeso),
    Libre(PreguntaLibre),
    SolaRespuesta(PreguntaSolaRespuesta),
    SiNo(PreguntaSiNo),
}

impl Pregunta {
    /// Retorna el ID de la pregunta independientemente de su tipo.
    #[must_use]
    pub fn id(&self) -> Id {
        match self {
            Self::AlternativaUnica(p) => p.id(),
            Self::AlternativaConPeso(p) => p.id(),
            Self::Libre(p) => p.id(),
            Self::SolaRespuesta(p) => p.id(),
            Self::SiNo(p) => p.id(),
        }
    }

    /// Retorna el contenido de la pregunta.
    #[must_use]
    pub fn contenido(&self) -> &SimpleName {
        match self {
            Self::AlternativaUnica(p) => p.contenido(),
            Self::AlternativaConPeso(p) => p.contenido(),
            Self::Libre(p) => p.contenido(),
            Self::SolaRespuesta(p) => p.contenido(),
            Self::SiNo(p) => p.contenido(),
        }
    }

    /// Retorna la referencia a imagen si existe.
    #[must_use]
    pub fn imagen(&self) -> Option<&ImagenRef> {
        match self {
            Self::AlternativaUnica(p) => p.imagen(),
            Self::AlternativaConPeso(p) => p.imagen(),
            Self::Libre(p) => p.imagen(),
            Self::SolaRespuesta(p) => p.imagen(),
            Self::SiNo(p) => p.imagen(),
        }
    }

    /// Retorna la etiqueta de la pregunta.
    #[must_use]
    pub fn etiqueta(&self) -> &Etiqueta {
        match self {
            Self::AlternativaUnica(p) => p.etiqueta(),
            Self::AlternativaConPeso(p) => p.etiqueta(),
            Self::Libre(p) => p.etiqueta(),
            Self::SolaRespuesta(p) => p.etiqueta(),
            Self::SiNo(p) => p.etiqueta(),
        }
    }

    /// Retorna el nombre del tipo de pregunta como string.
    #[must_use]
    pub fn tipo_nombre(&self) -> &'static str {
        match self {
            Self::AlternativaUnica(_) => "alternativa_unica",
            Self::AlternativaConPeso(_) => "alternativa_peso",
            Self::Libre(_) => "libre",
            Self::SolaRespuesta(_) => "sola_respuesta",
            Self::SiNo(_) => "si_o_no",
        }
    }

    /// Verifica si es una pregunta de alternativa única.
    #[must_use]
    pub fn es_alternativa_unica(&self) -> bool {
        matches!(self, Self::AlternativaUnica(_))
    }

    /// Verifica si es una pregunta con peso.
    #[must_use]
    pub fn es_alternativa_con_peso(&self) -> bool {
        matches!(self, Self::AlternativaConPeso(_))
    }

    /// Verifica si es una pregunta libre.
    #[must_use]
    pub fn es_libre(&self) -> bool {
        matches!(self, Self::Libre(_))
    }

    /// Verifica si es una pregunta de sola respuesta.
    #[must_use]
    pub fn es_sola_respuesta(&self) -> bool {
        matches!(self, Self::SolaRespuesta(_))
    }

    /// Verifica si es una pregunta de sí o no.
    #[must_use]
    pub fn es_si_no(&self) -> bool {
        matches!(self, Self::SiNo(_))
    }

    /// Intenta obtener la pregunta como alternativa única.
    #[must_use]
    pub fn as_alternativa_unica(&self) -> Option<&PreguntaAlternativaUnica> {
        match self {
            Self::AlternativaUnica(p) => Some(p),
            _ => None,
        }
    }

    /// Intenta obtener la pregunta como alternativa con peso.
    #[must_use]
    pub fn as_alternativa_con_peso(&self) -> Option<&PreguntaAlternativaConPeso> {
        match self {
            Self::AlternativaConPeso(p) => Some(p),
            _ => None,
        }
    }

    /// Intenta obtener la pregunta como libre.
    #[must_use]
    pub fn as_libre(&self) -> Option<&PreguntaLibre> {
        match self {
            Self::Libre(p) => Some(p),
            _ => None,
        }
    }

    /// Intenta obtener la pregunta como sola respuesta.
    #[must_use]
    pub fn as_sola_respuesta(&self) -> Option<&PreguntaSolaRespuesta> {
        match self {
            Self::SolaRespuesta(p) => Some(p),
            _ => None,
        }
    }

    /// Intenta obtener la pregunta como sí o no.
    #[must_use]
    pub fn as_si_no(&self) -> Option<&PreguntaSiNo> {
        match self {
            Self::SiNo(p) => Some(p),
            _ => None,
        }
    }
}

impl Entity for Pregunta {
    fn id(&self) -> Id {
        self.id()
    }
}

// Conversiones From para facilitar la creación del sum type
impl From<PreguntaAlternativaUnica> for Pregunta {
    fn from(p: PreguntaAlternativaUnica) -> Self {
        Self::AlternativaUnica(p)
    }
}

impl From<PreguntaAlternativaConPeso> for Pregunta {
    fn from(p: PreguntaAlternativaConPeso) -> Self {
        Self::AlternativaConPeso(p)
    }
}

impl From<PreguntaLibre> for Pregunta {
    fn from(p: PreguntaLibre) -> Self {
        Self::Libre(p)
    }
}

impl From<PreguntaSolaRespuesta> for Pregunta {
    fn from(p: PreguntaSolaRespuesta) -> Self {
        Self::SolaRespuesta(p)
    }
}

impl From<PreguntaSiNo> for Pregunta {
    fn from(p: PreguntaSiNo) -> Self {
        Self::SiNo(p)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::value_object::{
        Alternativa, AlternativaClave, AlternativasMultiples, Puntaje, PuntajeAlternativaUnica,
    };

    fn crear_alternativas_ejemplo() -> AlternativasMultiples {
        let items = vec![
            Alternativa::new(AlternativaClave::A, "Opción A".to_string()),
            Alternativa::new(AlternativaClave::B, "Opción B".to_string()),
            Alternativa::new(AlternativaClave::C, "Opción C".to_string()),
        ];
        AlternativasMultiples::new(items).unwrap()
    }

    #[test]
    fn test_sum_type_tipo_nombre() {
        let contenido = SimpleName::new("Test".to_string()).unwrap();

        let libre = Pregunta::Libre(PreguntaLibre::new(contenido.clone(), None, Etiqueta::No));
        assert_eq!(libre.tipo_nombre(), "libre");

        let alternativas = crear_alternativas_ejemplo();
        let puntaje = PuntajeAlternativaUnica::new(AlternativaClave::A, Puntaje::uno()).unwrap();
        let unica = Pregunta::AlternativaUnica(PreguntaAlternativaUnica::new(
            contenido.clone(),
            None,
            Etiqueta::No,
            alternativas,
            puntaje,
        ));
        assert_eq!(unica.tipo_nombre(), "alternativa_unica");
    }

    #[test]
    fn test_from_conversions() {
        let contenido = SimpleName::new("Test".to_string()).unwrap();
        let libre = PreguntaLibre::new(contenido, None, Etiqueta::No);

        let pregunta: Pregunta = libre.into();
        assert!(pregunta.es_libre());
    }

    #[test]
    fn test_as_methods() {
        let contenido = SimpleName::new("Test".to_string()).unwrap();
        let libre = PreguntaLibre::new(contenido, None, Etiqueta::No);
        let pregunta: Pregunta = libre.into();

        assert!(pregunta.as_libre().is_some());
        assert!(pregunta.as_alternativa_unica().is_none());
    }

    #[test]
    fn test_contenido_comun() {
        let contenido = SimpleName::new("¿Pregunta de prueba?".to_string()).unwrap();
        let libre = PreguntaLibre::new(contenido.clone(), None, Etiqueta::No);
        let pregunta: Pregunta = libre.into();

        assert_eq!(pregunta.contenido().as_str(), contenido.as_str());
    }
}
