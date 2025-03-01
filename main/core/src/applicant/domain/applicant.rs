use crate::applicant::domain::errors::ApplicantError;

/// El número de documento del postulante (p. ej., identificación nacional, pasaporte). El tipo
/// y formato específicos de este número dependerán de los requisitos de la aplicación.
/// Esta propiedad tambien debe ser único en el contexto de la aplicación.
pub struct DocumentNumber(String);

impl DocumentNumber {
    pub fn new(value: String) -> Result<Self, ApplicantError> {
        if value.trim().is_empty() {
            return Err(ApplicantError::InvalidDocumentNumber);
        }

        Ok(DocumentNumber(value))
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

/// Representa un nombre completo de solicitante validado.
pub struct FullName {
    /// Todos los nombres del postulante.
    name: String,

    /// Primer apellido del postulante
    first_lastname: String,

    /// Segundo apellido del postulante, esta propiedad no necesariamente sera obligatoria,
    /// dependiendo del contexto en el cual se implementa.
    second_lastname: String,
}

impl FullName {
    pub fn new(
        name: String,
        first_lastname: String,
        second_lastname: String,
    ) -> Result<Self, ApplicantError> {
        if name.trim().is_empty() || first_lastname.trim().is_empty() {
            return Err(ApplicantError::InvalidName);
        }
        Ok(FullName {
            name,
            first_lastname,
            second_lastname,
        })
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn first_lastname(&self) -> &String {
        &self.first_lastname
    }

    pub fn second_lastname(&self) -> &String {
        &self.second_lastname
    }
}

/// Representa el ID unico del postulante
#[derive(Debug, PartialEq)]
pub struct ApplicantID(pub String);

/// Representa al postulante para obtener ls _licencia de conducir_.
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
        let doc_num = DocumentNumber::new("12345678".to_string()).unwrap();
        let full_name =
            FullName::new("John".to_string(), "Doe".to_string(), "Smith".to_string()).unwrap();
        let applicant = Applicant::new(
            ApplicantID("c3299858-7bd5-4dce-b421-281d3177d45a".to_string()),
            doc_num,
            full_name,
        );
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
