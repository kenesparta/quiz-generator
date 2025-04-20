use crate::applicant::application::errors::{ApplicantOperationError, ApplicantPutError};
use crate::applicant::domain::applicant::{Applicant, ApplicantID};
use crate::applicant::domain::document::DocumentNumber;
use crate::applicant::domain::errors::ApplicantError;
use crate::applicant::domain::fullname::FullName;

pub struct ApplicantSignUpperDTO {
    pub name: String,
    pub first_lastname: String,
    pub second_lastname: String,
    pub document: String,
}

pub struct ApplicantSignUpper {
    pub applicant_list: Vec<Applicant>,
}

impl ApplicantSignUpper {
    pub fn new() -> Result<Self, ApplicantError> {
        Ok(ApplicantSignUpper {
            applicant_list: Vec::new(),
        })
    }

    pub fn insert_applicant(
        &mut self,
        id: String,
        applicant_request_dto: ApplicantSignUpperDTO,
    ) -> Result<(), ApplicantOperationError> {
        let applicant = Applicant::new(
            ApplicantID::new(id.to_string())?,
            DocumentNumber::new(applicant_request_dto.document)?,
            FullName::new(
                applicant_request_dto.name,
                applicant_request_dto.first_lastname,
                applicant_request_dto.second_lastname,
            )?,
        );

        if self
            .applicant_list
            .iter()
            .any(|a| a.id().value() == id.as_str())
        {
            return Err(ApplicantOperationError::InsertionError(
                ApplicantPutError::ApplicantAlreadyExist,
            ));
        }

        Ok(self.applicant_list.push(applicant))
    }
}
