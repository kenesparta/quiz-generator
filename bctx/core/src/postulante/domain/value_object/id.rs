use quizz_common::domain::value_objects::id::{ID, IdError};
use std::fmt;

/// Representa el ID unico del postulante
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PostulanteID {
    id: ID,
}

impl fmt::Display for PostulanteID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl PostulanteID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id).map(|id| PostulanteID { id })
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}

#[cfg(test)]
mod test_id {
    use super::*;

    #[test]
    fn test_empty_postulante_id() {
        let result = PostulanteID::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_postulante_id() {
        let result = PostulanteID::new("not-a-uuid");
        assert!(result.is_err());
    }
}
