use crate::pregunta::domain::entity::pregunta::PreguntaEntity;

#[derive(Debug, Clone)]
pub struct ListaDePreguntas(Vec<PreguntaEntity>);

impl ListaDePreguntas {
    pub fn new(preguntas: Vec<PreguntaEntity>) -> Self {
        Self(preguntas)
    }

    pub fn preguntas(&self) -> &Vec<PreguntaEntity> {
        &self.0
    }
}
