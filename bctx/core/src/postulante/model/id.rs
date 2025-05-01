use quizz_common::domain::value_objects::id::{ID, IdError};
use quizz_common::domain::value_objects::id_type::IdType;

/// Representa el ID unico del postulante
/// Represents an ID specifically for Postulante entities
pub struct PostulanteID {
    id: ID,
}

impl PostulanteID {
    /// Creates a new PostulanteID from a string representation
    pub fn new(id: impl AsRef<str>) -> Result<Self, IdError> {
        ID::new(id, IdType::Postulante).map(|id| PostulanteID { id })
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
