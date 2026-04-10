use crate::admin::domain::error::admin::AdminError;
use crate::admin::domain::value_object::id::AdminID;
use crate::postulante::domain::value_object::documento::Documento;

/// Representa al administrador del sistema.
#[derive(Debug)]
pub struct Admin {
    pub id: AdminID,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub password: Option<String>,
}

impl Admin {
    pub fn new(
        id: String,
        nombre: String,
        primer_apellido: String,
        segundo_apellido: String,
        documento: String,
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
        let documento_vo = Documento::new(&documento)
            .map_err(|e| AdminError::DocumentoNoValido(e.to_string()))?;
        if password.trim().is_empty() {
            return Err(AdminError::PasswordVacio);
        }

        Ok(Admin {
            id,
            nombre: nombre.trim().to_string(),
            primer_apellido: primer_apellido.trim().to_string(),
            segundo_apellido: segundo_apellido.trim().to_string(),
            documento: documento_vo.value().clone(),
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

    fn valid_documento() -> String {
        "11223344".to_string()
    }

    #[test]
    fn test_new_success() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            valid_documento(),
            valid_password(),
        );
        assert!(result.is_ok());
        let admin = result.unwrap();
        assert_eq!(admin.nombre, "Carlos");
        assert_eq!(admin.primer_apellido, "Martinez");
        assert_eq!(admin.documento, "11223344");
    }

    #[test]
    fn test_invalid_id() {
        let result = Admin::new(
            "not-a-valid-uuid".to_string(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            valid_documento(),
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
            valid_documento(),
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
            valid_documento(),
            valid_password(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::NombreNoValido(_)));
    }

    #[test]
    fn test_empty_documento() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            AdminError::DocumentoNoValido(_)
        ));
    }

    #[test]
    fn test_invalid_documento() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            "12".to_string(),
            valid_password(),
        );
        assert!(matches!(
            result.unwrap_err(),
            AdminError::DocumentoNoValido(_)
        ));
    }

    #[test]
    fn test_empty_password() {
        let result = Admin::new(
            valid_id(),
            "Carlos".to_string(),
            "Martinez".to_string(),
            "Lopez".to_string(),
            valid_documento(),
            "".to_string(),
        );
        assert!(matches!(result.unwrap_err(), AdminError::PasswordVacio));
    }
}
