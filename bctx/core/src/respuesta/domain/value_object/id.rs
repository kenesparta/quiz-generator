use crate::pregunta::domain::value_object::id::PreguntaID;
use quizz_common::domain::value_objects::id::{ID, IdError};
use quizz_common::domain::value_objects::id_type::IdType;
use std::fmt;

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
        ID::new(id, IdType::Respuesta).map(|id| RespuestaID { id })
    }

    pub fn new_v4() -> Self {
        RespuestaID {
            id: ID::new_v4(IdType::Respuesta),
        }
    }

    pub fn value(&self) -> &ID {
        &self.id
    }
}
