use crate::pregunta::domain::error::alternativa::AlternativaError;
use std::fmt;
use std::str::FromStr;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
pub enum Alternativa {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    Si,
    No,
}

impl fmt::Display for Alternativa {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Alternativa::A => write!(f, "A"),
            Alternativa::B => write!(f, "B"),
            Alternativa::C => write!(f, "C"),
            Alternativa::D => write!(f, "D"),
            Alternativa::E => write!(f, "E"),
            Alternativa::F => write!(f, "F"),
            Alternativa::G => write!(f, "G"),
            Alternativa::Si => write!(f, "SI"),
            Alternativa::No => write!(f, "NO"),
        }
    }
}

impl FromStr for Alternativa {
    type Err = AlternativaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Alternativa::A),
            "B" => Ok(Alternativa::B),
            "C" => Ok(Alternativa::C),
            "D" => Ok(Alternativa::D),
            "E" => Ok(Alternativa::E),
            "F" => Ok(Alternativa::F),
            "G" => Ok(Alternativa::G),
            "SI" => Ok(Alternativa::Si),
            "NO" => Ok(Alternativa::No),
            _ => Err(AlternativaError::NoValido),
        }
    }
}
