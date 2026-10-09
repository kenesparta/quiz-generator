use std::fmt;
use thiserror::Error;
use uuid::Uuid;

/// Error types related to ID validation
#[derive(Debug, Error)]
pub enum IdError {
    #[error("ID cannot be empty")]
    IdVacio,

    #[error("Invalid ID format")]
    FormatoNoValido,
}

/// Un UUID validado. El newtype de cada agregado (`ExamenID`, `PostulanteID`...) dice de qué
/// es el id; aquí solo está el valor, así que es `Copy` y sirve de clave.
///
/// Acepta cualquier forma que entienda [`Uuid`] (mayúsculas, sin guiones, entre llaves) y se
/// muestra siempre en la canónica (minúsculas con guiones), que es la que se guarda: un id
/// enviado en mayúsculas encuentra el mismo registro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ID(Uuid);

impl ID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        if id.is_empty() {
            return Err(IdError::IdVacio);
        }
        Uuid::parse_str(id)
            .map(ID)
            .map_err(|_| IdError::FormatoNoValido)
    }

    pub fn new_v4() -> Self {
        ID(Uuid::new_v4())
    }

    pub fn value(&self) -> String {
        self.0.to_string()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.0
    }
}

impl fmt::Display for ID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_id() {
        assert!(matches!(ID::new(""), Err(IdError::IdVacio)));
    }

    #[test]
    fn test_invalid_format() {
        assert!(matches!(
            ID::new("not-a-uuid"),
            Err(IdError::FormatoNoValido)
        ));
    }

    #[test]
    fn test_valid_id() {
        assert!(ID::new("550e8400-e29b-41d4-a716-446655440000").is_ok());
    }

    #[test]
    fn cualquier_forma_del_uuid_se_muestra_en_la_canonica() {
        let canonica = "550e8400-e29b-41d4-a716-446655440000";
        for forma in [
            canonica,
            "550E8400-E29B-41D4-A716-446655440000",
            "550e8400e29b41d4a716446655440000",
            "{550e8400-e29b-41d4-a716-446655440000}",
        ] {
            let id = ID::new(forma).unwrap();
            assert_eq!(id.to_string(), canonica, "{forma}");
            assert_eq!(id.value(), canonica, "{forma}");
            assert_eq!(id, ID::new(canonica).unwrap());
        }
    }
}
