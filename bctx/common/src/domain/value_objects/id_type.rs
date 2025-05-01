use std::fmt;

/// Type marker for different ID types in the application
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum IdType {
    Postulante,
    Custom(String),
}

impl fmt::Display for IdType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            IdType::Postulante => write!(f, "Postulante"),
            IdType::Custom(name) => write!(f, "{}", name),
        }
    }
}
