mod alternativa_unica;
mod con_peso;
mod si_no;
mod sola_respuesta;

pub use alternativa_unica::PuntajeAlternativaUnica;
pub use con_peso::PuntajeConPeso;
pub use si_no::PuntajeSiNo;
pub use sola_respuesta::PuntajeSolaRespuesta;

use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq)]
pub enum PuntajeError {
    #[error("El puntaje no puede ser negativo: {0}")]
    Negativo(f32),

    #[error("Puntaje vacío")]
    Vacio,

    #[error("Se requiere exactamente una respuesta correcta")]
    RequiereUnaRespuesta,

    #[error("La clave {0} no existe en las alternativas")]
    ClaveNoExiste(String),

    #[error("Se requiere puntaje para SI y NO")]
    RequiereSiNo,
}

/// Value Object que representa un puntaje válido (no negativo).
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct Puntaje {
    valor: f32,
}

impl Puntaje {
    /// Crea un nuevo puntaje.
    ///
    /// # Errors
    ///
    /// Retorna error si el valor es negativo.
    pub fn new(valor: f32) -> Result<Self, PuntajeError> {
        Self::es_menor_que_cero(valor)?;
        Ok(Self { valor })
    }

    /// Crea un puntaje de cero.
    #[must_use]
    pub const fn cero() -> Self {
        Self { valor: 0.0 }
    }

    /// Crea un puntaje de uno.
    #[must_use]
    pub const fn uno() -> Self {
        Self { valor: 1.0 }
    }

    #[must_use]
    pub fn valor(&self) -> f32 {
        self.valor
    }

    pub fn es_menor_que_cero(valor: f32) -> Result<(), PuntajeError> {
        if valor < 0.0 {
            return Err(PuntajeError::Negativo(valor));
        }

        Ok(())
    }

    pub fn sumar(&self, otro: &Self) -> Result<Self, PuntajeError> {
        Self::new(self.valor + otro.valor)
    }
}

impl Default for Puntaje {
    fn default() -> Self {
        Self::cero()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_puntaje_valido() {
        assert!(Puntaje::new(0.0).is_ok());
        assert!(Puntaje::new(1.0).is_ok());
        assert!(Puntaje::new(100.5).is_ok());
    }

    #[test]
    fn test_puntaje_negativo() {
        assert!(matches!(Puntaje::new(-1.0), Err(PuntajeError::Negativo(_))));
    }

    #[test]
    fn test_puntaje_constantes() {
        assert_eq!(Puntaje::cero().valor(), 0.0);
        assert_eq!(Puntaje::uno().valor(), 1.0);
    }
}
