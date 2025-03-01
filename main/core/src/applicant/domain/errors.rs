use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum ApplicantError {
    #[error("Duplicate applicant ID")]
    InvalidName,
    
    #[error("Duplicate applicant ID")]
    InvalidDocumentNumber,
}

#[derive(Error, Debug, PartialEq)]
pub enum ApplicantRegisterError {
    #[error("Duplicate applicant ID")]
    InvalidName,

    #[error("Duplicate applicant ID")]
    InvalidDocumentNumber,
}
