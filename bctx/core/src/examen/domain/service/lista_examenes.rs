use crate::examen::domain::entity::examen::Examen;

#[derive(Debug, Clone)]
pub struct ListaDeExamenes(Vec<Examen>);

impl ListaDeExamenes {
    pub fn new(examenes: Vec<Examen>) -> Self {
        Self(examenes)
    }

    pub fn examenes(&self) -> &Vec<Examen> {
        &self.0
    }
}
