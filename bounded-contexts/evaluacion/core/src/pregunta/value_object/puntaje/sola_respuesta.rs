use super::puntaje_base::Puntaje;

/// Value Object que representa el puntaje de una pregunta de sola respuesta.
///
/// Contiene la respuesta correcta esperada y el puntaje que se otorga
/// si la respuesta del postulante coincide.
#[derive(Debug, Clone, PartialEq)]
pub struct PuntajeSolaRespuesta {
    respuesta_correcta: String,
    puntaje: Puntaje,
}

impl PuntajeSolaRespuesta {
    /// Crea un nuevo puntaje de sola respuesta.
    #[must_use]
    pub fn new(respuesta_correcta: String, puntaje: Puntaje) -> Self {
        Self {
            respuesta_correcta,
            puntaje,
        }
    }

    #[must_use]
    pub fn respuesta_correcta(&self) -> &str {
        &self.respuesta_correcta
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
        let puntaje = PuntajeSolaRespuesta::new("París".to_string(), Puntaje::uno());
        assert_eq!(puntaje.respuesta_correcta(), "París");
        assert_eq!(puntaje.puntaje().valor(), 1.0);
    }
}
