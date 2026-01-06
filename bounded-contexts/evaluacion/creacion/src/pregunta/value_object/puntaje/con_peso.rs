use std::collections::HashMap;

use crate::pregunta::value_object::AlternativaClave;
use crate::{Puntaje, PuntajeError};

/// Value Object que representa el puntaje de una pregunta con peso por alternativa.
///
/// Cada alternativa puede tener un puntaje diferente.
#[derive(Debug, Clone, PartialEq)]
pub struct PuntajeConPeso {
    puntajes: HashMap<AlternativaClave, Puntaje>,
}

impl PuntajeConPeso {
    /// Crea un nuevo puntaje con peso.
    ///
    /// # Errors
    ///
    /// Retorna error si el mapa está vacío.
    pub fn new(puntajes: HashMap<AlternativaClave, Puntaje>) -> Result<Self, PuntajeError> {
        if puntajes.is_empty() {
            return Err(PuntajeError::Vacio);
        }

        // Verificar que todas las claves sean de tipo múltiple
        for clave in puntajes.keys() {
            if !clave.es_multiple() {
                return Err(PuntajeError::ClaveNoExiste(clave.to_string()));
            }
        }

        Ok(Self { puntajes })
    }

    /// Crea puntajes con peso desde un vector de tuplas (clave, valor).
    pub fn from_vec(items: Vec<(AlternativaClave, f32)>) -> Result<Self, PuntajeError> {
        let mut puntajes = HashMap::new();
        for (clave, valor) in items {
            let puntaje = Puntaje::new(valor)?;
            puntajes.insert(clave, puntaje);
        }
        Self::new(puntajes)
    }

    #[must_use]
    pub fn puntajes(&self) -> &HashMap<AlternativaClave, Puntaje> {
        &self.puntajes
    }

    /// Obtiene el puntaje para una alternativa específica.
    #[must_use]
    pub fn obtener(&self, clave: AlternativaClave) -> Option<Puntaje> {
        self.puntajes.get(&clave).copied()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion() {
        let puntaje = PuntajeConPeso::from_vec(vec![
            (AlternativaClave::A, 0.0),
            (AlternativaClave::B, 1.0),
            (AlternativaClave::C, 2.0),
        ]);
        assert!(puntaje.is_ok());
    }

    #[test]
    fn test_vacio() {
        let result = PuntajeConPeso::new(HashMap::new());
        assert!(matches!(result, Err(PuntajeError::Vacio)));
    }
}
