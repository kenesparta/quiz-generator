use crate::autorizacion::domain::value_object::rol::Rol;
use crate::usuario::domain::error::usuario::UsuarioError;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct Usuario {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: Rol,
}

impl Usuario {
    pub fn new(
        id: String,
        nombre: String,
        email: String,
        password: String,
        rol: String,
    ) -> Result<Self, UsuarioError> {
        if nombre.trim().is_empty() {
            return Err(UsuarioError::NombreVacio);
        }

        if email.trim().is_empty() {
            return Err(UsuarioError::EmailVacio);
        }

        if !email.contains('@') {
            return Err(UsuarioError::EmailInvalido(email));
        }

        if password.trim().is_empty() {
            return Err(UsuarioError::PasswordVacio);
        }

        let rol = Rol::from_str(&rol).map_err(|_| UsuarioError::RolInvalido(rol))?;

        Ok(Self {
            id,
            nombre: nombre.trim().to_string(),
            email: email.trim().to_lowercase(),
            password,
            rol,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_usuario_valido() {
        let usuario = Usuario::new(
            "abc-123".to_string(),
            "Juan Perez".to_string(),
            "juan@example.com".to_string(),
            "hashed_password".to_string(),
            "psicologo".to_string(),
        );
        assert!(usuario.is_ok());
        let u = usuario.unwrap();
        assert_eq!(u.nombre, "Juan Perez");
        assert_eq!(u.email, "juan@example.com");
        assert_eq!(u.rol, Rol::Psicologo);
    }

    #[test]
    fn test_crear_usuario_nombre_vacio() {
        let result = Usuario::new(
            "abc-123".to_string(),
            "".to_string(),
            "juan@example.com".to_string(),
            "pass".to_string(),
            "admin".to_string(),
        );
        assert!(matches!(result, Err(UsuarioError::NombreVacio)));
    }

    #[test]
    fn test_crear_usuario_email_invalido() {
        let result = Usuario::new(
            "abc-123".to_string(),
            "Juan".to_string(),
            "not-an-email".to_string(),
            "pass".to_string(),
            "admin".to_string(),
        );
        assert!(matches!(result, Err(UsuarioError::EmailInvalido(_))));
    }

    #[test]
    fn test_crear_usuario_rol_invalido() {
        let result = Usuario::new(
            "abc-123".to_string(),
            "Juan".to_string(),
            "juan@example.com".to_string(),
            "pass".to_string(),
            "superadmin".to_string(),
        );
        assert!(matches!(result, Err(UsuarioError::RolInvalido(_))));
    }

    #[test]
    fn test_crear_usuario_password_vacio() {
        let result = Usuario::new(
            "abc-123".to_string(),
            "Juan".to_string(),
            "juan@example.com".to_string(),
            "  ".to_string(),
            "admin".to_string(),
        );
        assert!(matches!(result, Err(UsuarioError::PasswordVacio)));
    }
}
