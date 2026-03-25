use crate::admin::domain::error::admin::AdminError;
use crate::admin::domain::value_object::id::AdminID;

/// Representa al administrador del sistema.
#[derive(Debug)]
pub struct Admin {
    pub id: AdminID,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub email: String,
    pub password: Option<String>,
}

impl Admin {
    pub fn new(
        id: String,
        nombre: String,
        primer_apellido: String,
        segundo_apellido: String,
        email: String,
        password: String,
    ) -> Result<Self, AdminError> {
        let id = AdminID::new(&id)?;

        if nombre.trim().is_empty() {
            return Err(AdminError::NombreNoValido("nombre vacio".to_string()));
        }
        if primer_apellido.trim().is_empty() {
            return Err(AdminError::NombreNoValido(
                "primer apellido vacio".to_string(),
            ));
        }
        if email.trim().is_empty() {
            return Err(AdminError::EmailVacio);
        }
        if !email.contains('@') {
            return Err(AdminError::EmailNoValido(email));
        }
        if password.trim().is_empty() {
            return Err(AdminError::PasswordVacio);
        }

        Ok(Admin {
            id,
            nombre: nombre.trim().to_string(),
            primer_apellido: primer_apellido.trim().to_string(),
            segundo_apellido: segundo_apellido.trim().to_string(),
            email: email.trim().to_lowercase(),
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
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "carlos@example.com".to_string(),
            valid_password(),
        );
        assert!(result.is_ok());
        let admin = result.unwrap();
        assert_eq!(admin.nombre, "Carlos");
        assert_eq!(admin.primer_apellido, "Martinez");
        assert_eq!(admin.email, "carlos@example.com");
    }

    #[test]
    fn test_invalid_id() {
        let result = Admin::new(
            "not-a-valid-uuid".to_string(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "carlos@example.com".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            AdminError::AdminIdError(IdError::FormatoNoValido(_))
        ));
    }

    #[test]
    fn test_empty_nombre() {
        let result = Admin::new(
            valid_id(),
            "".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "carlos@example.com".to_string(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::NombreNoValido(_)));
    }

    #[test]
    fn test_empty_primer_apellido() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "  ".to_string(),
            "Lopez".to_string(),
            "carlos@example.com".to_string(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::NombreNoValido(_)));
    }

    #[test]
    fn test_empty_email() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "".to_string(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::EmailVacio));
    }

    #[test]
    fn test_invalid_email() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "carlos-sin-arroba.com".to_string(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::EmailNoValido(_)));
    }

    #[test]
    fn test_empty_password() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "carlos@example.com".to_string(),
            "".to_string(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::PasswordVacio));
    }
}
