use quizz_common::domain::value_objects::id::{ID, IdError};
use quizz_common::domain::value_objects::id_type::IdType;
use std::fmt;

/// Representa el ID unico del psicologo
#[derive(Debug)]
pub struct PsicologoID {
    id: ID,
}

impl fmt::Display for PsicologoID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl PsicologoID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id, IdType::Custom("Psicologo".to_string())).map(|id| PsicologoID { id })
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}

#[cfg(test)]
mod test_id {
    use super::*;

    #[test]
    fn test_empty_psicologo_id() {
        let result = PsicologoID::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_psicologo_id() {
        let result = PsicologoID::new("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_psicologo_id() {
        let result = PsicologoID::new("872c8c81-9fab-494a-9267-799876261bcb");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_psicologo_id() {
        let id = PsicologoID::new("872c8c81-9fab-494a-9267-799876261bcb").unwrap();
        assert_eq!(format!("{}", id), "872c8c81-9fab-494a-9267-799876261bcb");
    }
}
