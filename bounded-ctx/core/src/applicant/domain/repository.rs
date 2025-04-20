use crate::applicant::domain::errors::ApplicantRegisterError;

pub trait CandidateRepository {
    fn register() -> Result<(), ApplicantRegisterError>;
}
