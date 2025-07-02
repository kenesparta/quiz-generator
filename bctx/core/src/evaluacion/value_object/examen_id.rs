use crate::evaluacion::value_object::id::EvaluacionID;
use quizz_common::domain::value_objects::id::ID;
use quizz_common::domain::value_objects::id_type::IdType;

pub struct ExamenIDs {
    examen_ids: Vec<ID>,
}

impl ExamenIDs {
    pub fn new(examen_ids: Vec<String>) -> Self {
        let ids: Vec<ID> = examen_ids
            .into_iter()
            .filter_map(|id_string| ID::new(&*id_string, IdType::Examen).ok())
            .collect();

        Self { examen_ids: ids }
    }

    pub fn examen_ids(&self) -> &Vec<ID> {
        &self.examen_ids
    }
}
