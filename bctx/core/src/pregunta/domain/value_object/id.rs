use quizz_common::domain::value_objects::id::{ID, IdError};
use std::fmt;

/// Representa el ID único de la pregunta
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PreguntaID {
    id: ID,
}

impl fmt::Display for PreguntaID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl PreguntaID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id).map(|id| PreguntaID { id })
    }

    pub fn new_v4() -> Self {
        PreguntaID { id: ID::new_v4() }
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}

#[cfg(test)]
mod test_id {
    use super::*;

    #[test]
    fn test_empty_pregunta_id() {
        let result = PreguntaID::new("");
        assert!(result.is_err());
    }

    #[test]
    fn test_invalid_format_pregunta_id() {
        let result = PreguntaID::new("not-a-uuid");
        assert!(result.is_err());
    }
}
