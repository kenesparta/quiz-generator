use crate::pregunta::domain::entity::pregunta::PreguntaProps;
use crate::pregunta::domain::error::pregunta::PreguntaError;

#[derive(Debug, Clone, PartialEq)]
pub struct PreguntaLibreProps {
    pub contenido: String,
    pub imagen_ref: Option<String>,
}

impl PreguntaProps for PreguntaLibreProps {
    fn contenido(&self) -> &str {
        &self.contenido
    }

    fn imagen_ref(&self) -> Option<&str> {
        self.imagen_ref.as_deref()
    }

    fn verificar_respuesta(&self, _respuesta: &str) -> Result<(), PreguntaError> {
        Ok(())
    }
}
