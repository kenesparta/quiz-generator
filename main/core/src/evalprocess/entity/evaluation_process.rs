use crate::applicant::entity::applicant::ApplicantID;

/// Se representa el proceso de seleccion el cual se va ha asignar a un postlante [ApplicantID].
/// Cada proceso de evaluacion [EvaluationProcess] podra contener varios examenes [Exam]
pub struct EvaluationProcess {
    pub id: EvaluationProcessID,
    pub candidate: ApplicantID,
    pub exams: Vec<Exam>,
    pub is_done: bool,
}

pub struct EvaluationProcessID(pub u64);

pub struct Exam {
    id: ExamID,
}

pub struct ExamID(pub u64);
