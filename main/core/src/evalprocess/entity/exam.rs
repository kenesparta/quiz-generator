use crate::candidate::entity::candidate::Candidate;

/// Se representa el proceso selecciovo entero el cual se va ha asignar a un candidato [Candidate].
/// Cada proceso de evaluacion (EvaluationProcess) podra contener varios examenes [Exam]
pub struct EvaluationProcess {
    pub candidate: Candidate,
    pub done: bool,
    pub exams: Vec<Exam>,
}

pub struct Exam {
    id: u64,
}