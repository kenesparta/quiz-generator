use crate::pregunta::domain::entity::pregunta::PreguntaProps;
use crate::pregunta::domain::error::pregunta::PreguntaError;

/// Este Tipo de preguntas son las que tienen una sola respuesta correcta
/// pero no son de eleccion multiple
#[derive(Debug, Clone, PartialEq)]
pub struct PreguntaSolaRespuestaProps {
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub respuesta_corecta: String,
}

impl PreguntaProps for PreguntaSolaRespuestaProps {
    fn contenido(&self) -> &str {
        &self.contenido
    }

    fn imagen_ref(&self) -> Option<&str> {
        self.imagen_ref.as_deref()
    }

    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError> {
        if respuesta.trim() != self.respuesta_corecta.trim() {
            return Err(PreguntaError::RespuestaIncorrecta);
        }

        Ok(())
    }
}
