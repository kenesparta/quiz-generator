use crate::applicant::entity::errors::ApplicantRegisterError;

pub trait CandidateRepository {
    fn register() -> Result<(), ApplicantRegisterError>;
}
