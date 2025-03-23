use crate::applicant::domain::document::DocumentNumber;
use crate::applicant::domain::errors::ApplicantError;
use crate::applicant::domain::fullname::FullName;
use uuid::Uuid;

/// Representa el ID unico del postulante
#[derive(Debug, PartialEq)]
pub struct ApplicantID(String);

impl ApplicantID {
    pub fn new(id: String) -> Result<Self, ApplicantError> {
        let applicant_id = ApplicantID(id);
        applicant_id.ensure_applicant_id_is_valid()?;
        Ok(applicant_id)
    }

    pub fn ensure_applicant_id_is_valid(&self) -> Result<(), ApplicantError> {
        if self.0.trim().is_empty() {
            return Err(ApplicantError::ApplicantIDisEmpty);
        }

        if Uuid::parse_str(&self.0).is_err() {
            return Err(ApplicantError::InvalidApplicantId);
        }

        Ok(())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

/// Representa al postulante para obtener la _licencia de conducir_.
pub struct Applicant {
    id: ApplicantID,
    document_number: DocumentNumber,
    full_name: FullName,
}

impl Applicant {
    pub fn new(id: ApplicantID, document_number: DocumentNumber, full_name: FullName) -> Self {
        Applicant {
            id,
            document_number,
            full_name,
        }
    }

    pub fn id(&self) -> &ApplicantID {
        &self.id
    }

    pub fn document_number(&self) -> &DocumentNumber {
        &self.document_number
    }

    pub fn full_name(&self) -> &FullName {
        &self.full_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_document_number_new_valid() {
        let doc_num = DocumentNumber::new("12345678".to_string());
        assert!(doc_num.is_ok());
        assert_eq!(doc_num.unwrap().value(), "12345678");
    }

    #[test]
    fn test_document_number_new_invalid() {
        let doc_num = DocumentNumber::new("  ".to_string());
        assert!(doc_num.is_err());
        assert_eq!(doc_num.err(), Some(ApplicantError::InvalidDocumentNumber));
    }

    #[test]
    fn test_full_name_new_valid() {
        let full_name = FullName::new("John".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(full_name.is_ok());
        let full_name = full_name.unwrap();
        assert_eq!(full_name.name(), "John");
        assert_eq!(full_name.first_lastname(), "Doe");
        assert_eq!(full_name.second_lastname(), "Smith");
    }

    #[test]
    fn test_full_name_new_invalid_name() {
        let full_name = FullName::new("  ".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(full_name.is_err());
        assert_eq!(full_name.err(), Some(ApplicantError::InvalidName));
    }

    #[test]
    fn test_full_name_new_invalid_first_lastname() {
        let full_name = FullName::new("John".to_string(), "  ".to_string(), "Smith".to_string());
        assert!(full_name.is_err());
        assert_eq!(full_name.err(), Some(ApplicantError::InvalidName));
    }

    #[test]
    fn test_applicant_new() {
        let applicant =
            ApplicantID::new("c3299858-7bd5-4dce-b421-281d3177d45a".to_string()).unwrap();
        let doc_num = DocumentNumber::new("12345678".to_string()).unwrap();
        let full_name =
            FullName::new("John".to_string(), "Doe".to_string(), "Smith".to_string()).unwrap();
        let applicant = Applicant::new(applicant, doc_num, full_name);
        assert_eq!(
            applicant.id,
            ApplicantID("c3299858-7bd5-4dce-b421-281d3177d45a".to_string())
        );
        assert_eq!(applicant.document_number.value(), "12345678");
        assert_eq!(applicant.full_name.name(), "John");
        assert_eq!(applicant.full_name.first_lastname(), "Doe");
        assert_eq!(applicant.full_name.second_lastname(), "Smith");
    }
}
