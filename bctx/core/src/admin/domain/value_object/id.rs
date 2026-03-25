use quizz_common::domain::value_objects::id::{ID, IdError};
use quizz_common::domain::value_objects::id_type::IdType;
use std::fmt;

/// Representa el ID unico del administrador
#[derive(Debug)]
pub struct AdminID {
    id: ID,
}

impl fmt::Display for AdminID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl AdminID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id, IdType::Custom("Admin".to_string())).map(|id| AdminID { id })
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}

#[cfg(test)]
mod test_id {
    use super::*;

    #[test]
    fn test_empty_admin_id() {
        let result = AdminID::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_admin_id() {
        let result = AdminID::new("not-a-uuid");
        assert!(result.is_err());
    }

    #[test]
    fn test_valid_admin_id() {
        let result = AdminID::new("872c8c81-9fab-494a-9267-799876261bcb");
        assert!(result.is_ok());
    }

    #[test]
    fn test_display_admin_id() {
        let id = AdminID::new("872c8c81-9fab-494a-9267-799876261bcb").unwrap();
        assert_eq!(format!("{}", id), "872c8c81-9fab-494a-9267-799876261bcb");
    }
}
