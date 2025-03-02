use crate::applicant::domain::errors::ApplicantError;

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
        let full_name = FullName {
            name,
            first_lastname,
            second_lastname,
        };
        full_name.ensure_name_is_correct()?;
        full_name.ensure_first_lastname_is_correct()?;
        full_name.ensure_second_lastname_is_correct()?;
        Ok(full_name)
    }

    fn ensure_name_is_correct(&self) -> Result<(), ApplicantError> {
        if self.name.trim().is_empty() {
            return Err(ApplicantError::InvalidName);
        }

        Ok(())
    }

    fn ensure_first_lastname_is_correct(&self) -> Result<(), ApplicantError> {
        if self.first_lastname.trim().is_empty() {
            return Err(ApplicantError::InvalidName);
        }

        Ok(())
    }

    fn ensure_second_lastname_is_correct(&self) -> Result<(), ApplicantError> {
        if self.second_lastname.trim().is_empty() {
            return Err(ApplicantError::InvalidName);
        }

        Ok(())
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
