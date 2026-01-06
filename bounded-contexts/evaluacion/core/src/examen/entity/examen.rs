use evaluacion_common::{Entity, Id, SimpleName, SimpleNameConfig};

use crate::examen::error::ExamenError;
use crate::pregunta::Pregunta;

/// Configuración para el título del examen (3-150 caracteres).
pub(super) const TITULO_CONFIG: SimpleNameConfig = SimpleNameConfig::new(3, 150);

/// Configuración para la descripción del examen (1-250 caracteres).
const DESCRIPCION_CONFIG: SimpleNameConfig = SimpleNameConfig::new(1, 250);

/// Configuración para las instrucciones del examen (1-500 caracteres).
const INSTRUCCIONES_CONFIG: SimpleNameConfig = SimpleNameConfig::new(1, 500);

/// Entidad que representa un examen.
///
/// Un examen contiene un conjunto de preguntas que serán respondidas
/// por los postulantes.
///
/// # Campos
///
/// - `titulo`: Obligatorio, 3-150 caracteres
/// - `descripcion`: Opcional, hasta 250 caracteres
/// - `instrucciones`: Opcional, hasta 500 caracteres
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Examen {
    pub(super) id: Id,
    pub(super) titulo: SimpleName,
    pub(super) descripcion: Option<SimpleName>,
    pub(super) instrucciones: Option<SimpleName>,
    pub(super) preguntas: Vec<Pregunta>,
}

impl Examen {
    /// Crea un nuevo examen.
    ///
    /// # Errors
    ///
    /// Retorna error si:
    /// - El título no cumple con las validaciones (3-150 caracteres)
    /// - La descripción no cumple con las validaciones (1-250 caracteres)
    /// - Las instrucciones no cumplen con las validaciones (1-500 caracteres)
    pub fn new(
        titulo: String,
        descripcion: Option<String>,
        instrucciones: Option<String>,
    ) -> Result<Self, ExamenError> {
        let titulo =
            SimpleName::with_config(titulo, TITULO_CONFIG).map_err(ExamenError::TituloInvalido)?;

        let descripcion = descripcion
            .map(|d| SimpleName::with_config(d, DESCRIPCION_CONFIG))
            .transpose()
            .map_err(ExamenError::DescripcionInvalida)?;

        let instrucciones = instrucciones
            .map(|i| SimpleName::with_config(i, INSTRUCCIONES_CONFIG))
            .transpose()
            .map_err(ExamenError::InstruccionesInvalidas)?;

        Ok(Self {
            id: Id::new(),
            titulo,
            descripcion,
            instrucciones,
            preguntas: Vec::new(),
        })
    }

    /// Crea un examen con un ID específico (para reconstrucción desde persistencia).
    ///
    /// # Errors
    ///
    /// Retorna error si:
    /// - El título no cumple con las validaciones (3-150 caracteres)
    /// - La descripción no cumple con las validaciones (1-250 caracteres)
    /// - Las instrucciones no cumplen con las validaciones (1-500 caracteres)
    pub fn with_id(
        id: Id,
        titulo: String,
        descripcion: Option<String>,
        instrucciones: Option<String>,
        preguntas: Vec<Pregunta>,
    ) -> Result<Self, ExamenError> {
        let titulo =
            SimpleName::with_config(titulo, TITULO_CONFIG).map_err(ExamenError::TituloInvalido)?;

        let descripcion = descripcion
            .map(|d| SimpleName::with_config(d, DESCRIPCION_CONFIG))
            .transpose()
            .map_err(ExamenError::DescripcionInvalida)?;

        let instrucciones = instrucciones
            .map(|i| SimpleName::with_config(i, INSTRUCCIONES_CONFIG))
            .transpose()
            .map_err(ExamenError::InstruccionesInvalidas)?;

        Ok(Self {
            id,
            titulo,
            descripcion,
            instrucciones,
            preguntas,
        })
    }

    #[must_use]
    pub fn titulo(&self) -> &SimpleName {
        &self.titulo
    }

    #[must_use]
    pub fn descripcion(&self) -> Option<&SimpleName> {
        self.descripcion.as_ref()
    }

    #[must_use]
    pub fn instrucciones(&self) -> Option<&SimpleName> {
        self.instrucciones.as_ref()
    }

    #[must_use]
    pub fn preguntas(&self) -> &[Pregunta] {
        &self.preguntas
    }

    #[must_use]
    pub fn cantidad_preguntas(&self) -> usize {
        self.preguntas.len()
    }

    #[must_use]
    pub fn esta_vacio(&self) -> bool {
        self.preguntas.is_empty()
    }
}

impl Entity for Examen {
    fn id(&self) -> Id {
        self.id
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn crear_examen_ejemplo() -> Examen {
        Examen::new(
            "Examen de Prueba".to_string(),
            Some("Descripción del examen".to_string()),
            Some("Lea cuidadosamente".to_string()),
        )
        .unwrap()
    }

    #[test]
    fn test_crear_examen() {
        let examen = crear_examen_ejemplo();
        assert_eq!(examen.titulo().as_str(), "Examen de Prueba");
        assert!(examen.esta_vacio());
    }

    #[test]
    fn test_crear_examen_sin_descripcion() {
        let examen = Examen::new("Examen Simple".to_string(), None, None).unwrap();

        assert!(examen.descripcion().is_none());
        assert!(examen.instrucciones().is_none());
    }

    #[test]
    fn test_titulo_minimo() {
        let examen = Examen::new("ABC".to_string(), None, None);
        assert!(examen.is_ok());
    }

    #[test]
    fn test_titulo_muy_corto() {
        let examen = Examen::new("AB".to_string(), None, None);
        assert!(matches!(examen, Err(ExamenError::TituloInvalido(_))));
    }

    #[test]
    fn test_titulo_muy_largo() {
        let examen = Examen::new("A".repeat(151), None, None);
        assert!(matches!(examen, Err(ExamenError::TituloInvalido(_))));
    }

    #[test]
    fn test_descripcion_limite() {
        let examen = Examen::new("Examen".to_string(), Some("A".repeat(250)), None);
        assert!(examen.is_ok());
    }

    #[test]
    fn test_descripcion_muy_larga() {
        let examen = Examen::new("Examen".to_string(), Some("A".repeat(251)), None);
        assert!(matches!(examen, Err(ExamenError::DescripcionInvalida(_))));
    }

    #[test]
    fn test_instrucciones_limite() {
        let examen = Examen::new("Examen".to_string(), None, Some("A".repeat(500)));
        assert!(examen.is_ok());
    }

    #[test]
    fn test_instrucciones_muy_largas() {
        let examen = Examen::new("Examen".to_string(), None, Some("A".repeat(501)));
        assert!(matches!(
            examen,
            Err(ExamenError::InstruccionesInvalidas(_))
        ));
    }
}
