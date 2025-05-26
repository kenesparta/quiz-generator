use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::id::PreguntaID;
use std::fmt::Debug;

pub trait Pregunta {
    fn id(&self) -> &PreguntaID;
    fn contenido(&self) -> &str;
    fn imagen_ref(&self) -> Option<&str>;
    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError>;
}

pub trait PreguntaProps: Clone + PartialEq + Debug {
    fn contenido(&self) -> &str;
    fn imagen_ref(&self) -> Option<&str>;
    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError>;
}

#[derive(Debug, Clone)]
pub struct PreguntaEntity<Props: PreguntaProps> {
    pub id: PreguntaID,
    pub props: Props,
}

impl<Props: PreguntaProps> PreguntaEntity<Props> {
    pub fn new(id: PreguntaID, props: Props) -> Self {
        Self { id, props }
    }
}

impl<Props: PreguntaProps> Pregunta for PreguntaEntity<Props> {
    fn id(&self) -> &PreguntaID {
        &self.id
    }

    fn contenido(&self) -> &str {
        self.props.contenido()
    }

    fn imagen_ref(&self) -> Option<&str> {
        self.props.imagen_ref()
    }

    fn verificar_respuesta(&self, respuesta: &str) -> Result<(), PreguntaError> {
        Ok(self.props.verificar_respuesta(respuesta)?)
    }
}
