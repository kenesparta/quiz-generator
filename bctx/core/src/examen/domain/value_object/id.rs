use quizz_common::domain::value_objects::id::{ID, IdError};
use std::fmt;

/// Representa el ID único del examen
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ExamenID {
    id: ID,
}

impl fmt::Display for ExamenID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl ExamenID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id).map(|id| ExamenID { id })
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}

#[cfg(test)]
mod test_id {
    use super::*;

    #[test]
    fn test_empty_examen_id() {
        let result = ExamenID::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_examen_id() {
        let result = ExamenID::new("not-a-uuid");
        assert!(result.is_err());
    }
}
