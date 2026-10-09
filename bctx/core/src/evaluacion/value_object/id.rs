use quizz_common::domain::value_objects::id::{ID, IdError};
use std::fmt;
use std::fmt::Formatter;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct EvaluacionID {
    id: ID,
}

impl fmt::Display for EvaluacionID {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl EvaluacionID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id).map(|id| EvaluacionID { id })
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}
