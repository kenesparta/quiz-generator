use crate::pregunta::domain::entity::pregunta::PreguntaEntity;

pub struct ListaDePreguntas {
    preguntas: Vec<PreguntaEntity>,
}

impl ListaDePreguntas {
    pub fn new(preguntas: Vec<PreguntaEntity>) -> Self {
        Self { preguntas }
    }
}
