use crate::pregunta::value_object::AlternativaClave;
use crate::{Puntaje, PuntajeError};

/// Value Object que representa el puntaje de una pregunta de alternativa única.
///
/// Solo una alternativa tiene puntaje (la respuesta correcta).
#[derive(Debug, Clone, PartialEq)]
pub struct PuntajeAlternativaUnica {
    respuesta_correcta: AlternativaClave,
    puntaje: Puntaje,
}

impl PuntajeAlternativaUnica {
    /// Crea un nuevo puntaje de alternativa única.
    pub fn new(
        respuesta_correcta: AlternativaClave,
        puntaje: Puntaje,
    ) -> Result<Self, PuntajeError> {
        if !respuesta_correcta.es_multiple() {
            return Err(PuntajeError::ClaveNoExiste(respuesta_correcta.to_string()));
        }
        Ok(Self {
            respuesta_correcta,
            puntaje,
        })
    }

    #[must_use]
    pub fn respuesta_correcta(&self) -> AlternativaClave {
        self.respuesta_correcta
    }

    #[must_use]
    pub fn puntaje(&self) -> Puntaje {
        self.puntaje
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion() {
        let puntaje = PuntajeAlternativaUnica::new(AlternativaClave::C, Puntaje::uno());
        assert!(puntaje.is_ok());
    }

    #[test]
    fn test_no_permite_clave_si_no() {
        let result = PuntajeAlternativaUnica::new(AlternativaClave::Si, Puntaje::uno());
        assert!(matches!(result, Err(PuntajeError::ClaveNoExiste(_))));
    }
}
