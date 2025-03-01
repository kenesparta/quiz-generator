use thiserror::Error;
use crate::applicant::domain::errors::ApplicantError;

#[derive(Error, Debug, PartialEq)]
pub enum ApplicantPutError {
    #[error("Duplicate applicant ID")]
    ApplicantDoesNotExist,

    #[error("Duplicate applicant ID")]
    ApplicantAlreadyExist,
}

#[derive(Error, Debug)]
pub enum ApplicantOperationError {
    #[error("Applicant validation error: {0}")]
    ValidationError(#[from] ApplicantError),

    #[error("Applicant insertion error: {0}")]
    InsertionError(#[from] ApplicantPutError),
}