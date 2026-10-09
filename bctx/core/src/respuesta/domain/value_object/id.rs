use quizz_common::domain::value_objects::id::{ID, IdError};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct RespuestaID {
    id: ID,
}

impl fmt::Display for RespuestaID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id.value())
    }
}

impl RespuestaID {
    pub fn new(id: &str) -> Result<Self, IdError> {
        ID::new(id).map(|id| RespuestaID { id })
    }

    pub fn new_v4() -> Self {
        RespuestaID { id: ID::new_v4() }
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}
