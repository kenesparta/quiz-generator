use crate::domain::value_objects::id_type::IdType;
use thiserror::Error;
use uuid::Uuid;

/// Error types related to ID validation
#[derive(Debug, Error)]
pub enum IdError {
    #[error("ID cannot be empty")]
    IdVacio,

    #[error("Invalid ID format for {0}")]
    FormatoNoValido(String),
}

/// Represents a typed unique ID that can be used throughout the application
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct ID {
    uuid: Uuid,
    id_type: IdType,
}

impl ID {
    pub fn new(id: &str, id_type: IdType) -> Result<Self, IdError> {
        if id.is_empty() {
            return Err(IdError::IdVacio);
        }

        let type_string = id_type.to_string();

        Uuid::parse_str(id)
            .map(|uuid| ID { uuid, id_type })
            .map_err(|_| IdError::FormatoNoValido(type_string))
    }

    pub fn value(&self) -> String {
        self.uuid.to_string()
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_id() {
        let result = ID::new("", IdType::Postulante);
        assert!(matches!(result, Err(IdError::IdVacio)));
    }

    #[test]
    fn test_invalid_format() {
        let result = ID::new("not-a-uuid", IdType::Postulante);
        assert!(matches!(result, Err(IdError::FormatoNoValido(_))));
    }

    #[test]
    fn test_valid_id() {
        let valid_uuid = "550e8400-e29b-41d4-a716-446655440000";
        let result = ID::new(valid_uuid, IdType::Postulante);
        assert!(result.is_ok());
    }
}
