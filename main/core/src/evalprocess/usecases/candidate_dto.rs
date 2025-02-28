use crate::applicant::entity::applicant::{Applicant, DocumentNumber, FullName};
use crate::applicant::entity::errors::ApplicantCreationError;

pub struct ApplicantDto {
    pub name: String,
    pub document_number: String,
    pub first_lastname: String,
    pub second_lastname: String,
}

impl ApplicantDto {
    pub fn new(
        name: String,
        document_number: String,
        first_lastname: String,
        second_lastname: String,
    ) -> Self {
        ApplicantDto {
            name,
            document_number,
            first_lastname,
            second_lastname,
        }
    }

    pub fn map_to_entity(self) -> Result<Applicant, ApplicantCreationError> {
        let document_number = DocumentNumber::new(self.document_number)?;
        let full_name = FullName::new(self.name, self.first_lastname, self.second_lastname)?;
        let applicant = Applicant {
            id: None,
            document_number,
            full_name,
        };
        Ok(applicant)
    }
}
