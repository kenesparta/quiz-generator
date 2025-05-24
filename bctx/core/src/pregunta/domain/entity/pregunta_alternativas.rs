use crate::pregunta::domain::entity::pregunta::PreguntaProps;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub struct PreguntaAlternativasProps {
    pub contenido: String,
    pub imagen_ref: Option<String>,

    // Esta es la llave de la respuesta correcta
    pub alternativa_correcta: String,

    // La lista de las alternativas creadas
    pub alternativas: HashMap<String, String>,
}

impl PreguntaProps for PreguntaAlternativasProps {
    fn contenido(&self) -> &str {
        &self.contenido
    }

    fn imagen_ref(&self) -> Option<&str> {
        self.imagen_ref.as_deref()
    }

    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError> {
        if !self.alternativas.contains_key(respuesta) {
            return Err(PreguntaError::RespuestaNoExiste);
        }

        if respuesta != self.alternativa_correcta {
            return Err(PreguntaError::RespuestaIncorrecta);
        }
        Ok(())
    }
}
