use crate::postulante::domain::error::grado_instruccion::GradoInstruccionError;
use std::str::FromStr;

#[derive(Debug)]
pub enum GradoInstruccion {
    Ninguno,
    Primaria,
    Secundaria,
    Superior,
    Posgrado,
}

impl fmt::Display for PostulanteID {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Assuming PostulanteID contains a value that can be displayed
        // If it's a wrapper around UUID, String, or other primitive type:
        write!(f, "{}", self.0)  // Adjust this based on your actual structure
    }
}


impl FromStr for GradoInstruccion {
    type Err = GradoInstruccionError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "ninguno" => Ok(Self::Ninguno),
            "primaria" => Ok(Self::Primaria),
            "secundaria" => Ok(Self::Secundaria),
            "superior" => Ok(Self::Superior),
            "posgrado" => Ok(Self::Posgrado),
            _ => Err(GradoInstruccionError::NoValido),
        }
    }
}
