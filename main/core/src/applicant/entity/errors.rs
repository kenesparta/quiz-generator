#[derive(Debug, PartialEq)]
pub enum ApplicantCreationError {
    InvalidName,
    InvalidDocumentNumber,
}

#[derive(Debug, PartialEq)]
pub enum ApplicantRegisterError {
    InvalidName,
    InvalidDocumentNumber,
}
