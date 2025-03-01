use crate::applicant::domain::applicant::{Applicant, ApplicantID, DocumentNumber, FullName};

pub struct ApplicantSignUpperDTO {
    pub name: String,
    pub first_lastname: String,
    pub second_lastname: String,
    pub document: String,
}

pub struct ApplicantSignUpper {
    pub applicant: Applicant,
}

impl ApplicantSignUpper {
    pub fn new(id: String, applicant_request_dto: ApplicantSignUpperDTO) -> Self {
        let doc_num = DocumentNumber::new(applicant_request_dto.document).unwrap();
        let full_name = FullName::new(
            applicant_request_dto.name,
            applicant_request_dto.first_lastname,
            applicant_request_dto.second_lastname,
        )
        .unwrap();
        let applicant = Applicant::new(ApplicantID(id.to_string()), doc_num, full_name);

        ApplicantSignUpper { applicant }
    }
}
