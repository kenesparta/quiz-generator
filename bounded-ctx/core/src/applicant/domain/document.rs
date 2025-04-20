use crate::applicant::domain::errors::ApplicantError;

/// El número de documento del postulante (p. ej., identificación nacional, pasaporte). El tipo
/// y formato específicos de este número dependerán de los requisitos de la aplicación.
/// Esta propiedad tambien debe ser único en el contexto de la aplicación.
pub struct DocumentNumber(String);

impl DocumentNumber {
    pub fn new(value: String) -> Result<Self, ApplicantError> {
        let document = DocumentNumber(value);
        document.ensure_document_number_is_valid()?;
        Ok(document)
    }

    pub fn ensure_document_number_is_valid(&self) -> Result<(), ApplicantError> {
        if self.0.trim().is_empty() {
            return Err(ApplicantError::InvalidDocumentNumber);
        }

        Ok(())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}
