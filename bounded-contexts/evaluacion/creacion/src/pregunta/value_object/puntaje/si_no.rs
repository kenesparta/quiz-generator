use crate::Puntaje;

/// Value Object que representa el puntaje de una pregunta Si/No.
#[derive(Debug, Clone, PartialEq)]
pub struct PuntajeSiNo {
    puntaje_si: Puntaje,
    puntaje_no: Puntaje,
}

impl PuntajeSiNo {
    /// Crea un nuevo puntaje Si/No.
    #[must_use]
    pub fn new(puntaje_si: Puntaje, puntaje_no: Puntaje) -> Self {
        Self {
            puntaje_si,
            puntaje_no,
        }
    }

    /// Crea un puntaje Si/No donde solo SI da puntos.
    pub fn solo_si(puntaje: Puntaje) -> Self {
        Self {
            puntaje_si: puntaje,
            puntaje_no: Puntaje::cero(),
        }
    }

    /// Crea un puntaje Si/No donde solo NO da puntos.
    pub fn solo_no(puntaje: Puntaje) -> Self {
        Self {
            puntaje_si: Puntaje::cero(),
            puntaje_no: puntaje,
        }
    }

    #[must_use]
    pub fn puntaje_si(&self) -> Puntaje {
        self.puntaje_si
    }

    #[must_use]
    pub fn puntaje_no(&self) -> Puntaje {
        self.puntaje_no
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion() {
        let puntaje = PuntajeSiNo::new(Puntaje::new(2.0).unwrap(), Puntaje::new(1.0).unwrap());
        assert_eq!(puntaje.puntaje_si().valor(), 2.0);
        assert_eq!(puntaje.puntaje_no().valor(), 1.0);
    }

    #[test]
    fn test_solo_si() {
        let puntaje = PuntajeSiNo::solo_si(Puntaje::uno());
        assert_eq!(puntaje.puntaje_si().valor(), 1.0);
        assert_eq!(puntaje.puntaje_no().valor(), 0.0);
    }

    #[test]
    fn test_solo_no() {
        let puntaje = PuntajeSiNo::solo_no(Puntaje::uno());
        assert_eq!(puntaje.puntaje_si().valor(), 0.0);
        assert_eq!(puntaje.puntaje_no().valor(), 1.0);
    }
}
