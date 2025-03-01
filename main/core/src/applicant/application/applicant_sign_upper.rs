use crate::applicant::application::errors::{ApplicantOperationError, ApplicantPutError};
use crate::applicant::domain::applicant::{Applicant, ApplicantID, DocumentNumber, FullName};
use crate::applicant::domain::errors::ApplicantError;
use std::cmp::PartialEq;

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
        let doc_num = DocumentNumber::new(applicant_request_dto.document)?;
        let full_name = FullName::new(
            applicant_request_dto.name,
            applicant_request_dto.first_lastname,
            applicant_request_dto.second_lastname,
        )?;
        let applicant = Applicant::new(ApplicantID(id.to_string()), doc_num, full_name);

        if self.applicant_list.iter().any(|a| a.id().0 == id) {
            return Err(ApplicantOperationError::InsertionError(
                ApplicantPutError::ApplicantAlreadyExist,
            ));
        }

        Ok(self.applicant_list.push(applicant))
    }
}
