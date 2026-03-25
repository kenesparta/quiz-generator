use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::domain::value_object::id::PsicologoID;

/// Representa al psicologo que administra y revisa las evaluaciones.
#[derive(Debug)]
pub struct Psicologo {
    pub id: PsicologoID,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub email: String,
    pub especialidad: String,
    pub password: Option<String>,
}

impl Psicologo {
    pub fn new(
        id: String,
        nombre: String,
        primer_apellido: String,
        segundo_apellido: String,
        email: String,
        especialidad: String,
        password: String,
    ) -> Result<Self, PsicologoError> {
        let id = PsicologoID::new(&id)?;

        if nombre.trim().is_empty() {
            return Err(PsicologoError::NombreNoValido("nombre vacio".to_string()));
        }
        if primer_apellido.trim().is_empty() {
            return Err(PsicologoError::NombreNoValido(
                "primer apellido vacio".to_string(),
            ));
        }
        if email.trim().is_empty() {
            return Err(PsicologoError::EmailVacio);
        }
        if !email.contains('@') {
            return Err(PsicologoError::EmailNoValido(email));
        }
        if especialidad.trim().is_empty() {
            return Err(PsicologoError::EspecialidadVacia);
        }
        if password.trim().is_empty() {
            return Err(PsicologoError::PasswordVacio);
        }

        Ok(Psicologo {
            id,
            nombre: nombre.trim().to_string(),
            primer_apellido: primer_apellido.trim().to_string(),
            segundo_apellido: segundo_apellido.trim().to_string(),
            email: email.trim().to_lowercase(),
            especialidad: especialidad.trim().to_string(),
            password: Some(password),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizz_common::domain::value_objects::id::IdError;

    fn valid_id() -> String {
        "872c8c81-9fab-494a-9267-799876261bcb".to_string()
    }

    fn valid_password() -> String {
        "$2a$12$b0a7aabc6PcLyAMKifb3pOCSwi8zgqf0ylujb8DgF3I1r.xn.Mrn2".to_string()
    }

    #[test]
    fn test_new_success() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(result.is_ok());
        let psicologo = result.unwrap();
        assert_eq!(psicologo.nombre, "Maria");
        assert_eq!(psicologo.primer_apellido, "Garcia");
        assert_eq!(psicologo.email, "maria@example.com");
        assert_eq!(psicologo.especialidad, "Psicologia Clinica");
    }

    #[test]
    fn test_invalid_id() {
        let result = Psicologo::new(
            "not-a-valid-uuid".to_string(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            PsicologoError::PsicologoIdError(IdError::FormatoNoValido(_))
        ));
    }

    #[test]
    fn test_empty_nombre() {
        let result = Psicologo::new(
            valid_id(),
            "".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            PsicologoError::NombreNoValido(_)
        ));
    }

    #[test]
    fn test_empty_primer_apellido() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "  ".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            PsicologoError::NombreNoValido(_)
        ));
    }

    #[test]
    fn test_empty_email() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), PsicologoError::EmailVacio));
    }

    #[test]
    fn test_invalid_email() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria-no-arroba.com".to_string(),
            "Psicologia Clinica".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            PsicologoError::EmailNoValido(_)
        ));
    }

    #[test]
    fn test_empty_especialidad() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            PsicologoError::EspecialidadVacia
        ));
    }

    #[test]
    fn test_empty_password() {
        let result = Psicologo::new(
            valid_id(),
            "Maria".to_string(),
            "Garcia".to_string(),
            "Lopez".to_string(),
            "maria@example.com".to_string(),
            "Psicologia Clinica".to_string(),
            "".to_string(),
        );
        assert!(matches!(result.unwrap_err(), PsicologoError::PasswordVacio));
    }
}
