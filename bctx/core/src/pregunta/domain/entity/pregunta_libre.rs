use crate::pregunta::domain::entity::pregunta::PreguntaProps;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoDePregunta;

#[derive(Debug, Clone, PartialEq)]
pub struct PreguntaLibreProps {
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub respuesta_libre: String,
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

    fn tipo() -> TipoDePregunta {
        TipoDePregunta::Libre
    }
}
