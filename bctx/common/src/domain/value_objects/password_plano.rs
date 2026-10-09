use std::fmt;
use thiserror::Error;

/// Mínimo de caracteres de una contraseña elegida por una persona.
pub const MIN_CARACTERES_PASSWORD: usize = 8;
/// bcrypt ignora lo que sigue al byte 72: dos contraseñas que solo difieren después de ahí
/// verificarían igual, así que no se aceptan más largas.
pub const MAX_BYTES_PASSWORD: usize = 72;

#[derive(Debug, Error, PartialEq)]
pub enum PasswordPlanoError {
    #[error("El password no puede estar vacio")]
    Vacio,

    #[error("El password debe tener al menos {MIN_CARACTERES_PASSWORD} caracteres")]
    MuyCorto,

    #[error("El password no puede superar los {MAX_BYTES_PASSWORD} bytes")]
    MuyLargo,
}

/// Una contraseña en texto plano, validada antes de calcular su hash.
///
/// La validación se hace sobre el texto plano: comprobar que el hash no esté vacío (lo que se
/// hacía antes) no detecta nada, porque bcrypt calcula el hash de "" sin error. `Debug` no
/// muestra el valor.
pub struct PasswordPlano(String);

impl PasswordPlano {
    /// # Errors
    ///
    /// [`PasswordPlanoError`] si está vacía (o solo tiene espacios), tiene menos de
    /// [`MIN_CARACTERES_PASSWORD`] caracteres o más de [`MAX_BYTES_PASSWORD`] bytes.
    pub fn new(valor: String) -> Result<Self, PasswordPlanoError> {
        if valor.trim().is_empty() {
            return Err(PasswordPlanoError::Vacio);
        }
        if valor.chars().count() < MIN_CARACTERES_PASSWORD {
            return Err(PasswordPlanoError::MuyCorto);
        }
        if valor.len() > MAX_BYTES_PASSWORD {
            return Err(PasswordPlanoError::MuyLargo);
        }
        Ok(Self(valor))
    }

    pub fn into_inner(self) -> String {
        self.0
    }
}

impl fmt::Debug for PasswordPlano {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PasswordPlano(***)")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rechaza_vacio_y_solo_espacios() {
        assert_eq!(
            PasswordPlano::new(String::new()).err(),
            Some(PasswordPlanoError::Vacio)
        );
        assert_eq!(
            PasswordPlano::new("        ".to_string()).err(),
            Some(PasswordPlanoError::Vacio)
        );
    }

    #[test]
    fn el_minimo_se_cuenta_en_caracteres() {
        let justo = "ñ".repeat(MIN_CARACTERES_PASSWORD);
        assert!(PasswordPlano::new(justo).is_ok());
        let corto = "ñ".repeat(MIN_CARACTERES_PASSWORD - 1);
        assert_eq!(
            PasswordPlano::new(corto).err(),
            Some(PasswordPlanoError::MuyCorto)
        );
    }

    #[test]
    fn el_maximo_se_cuenta_en_bytes() {
        let justo = "a".repeat(MAX_BYTES_PASSWORD);
        assert!(PasswordPlano::new(justo).is_ok());
        // 37 'ñ' son 74 bytes aunque solo sean 37 caracteres.
        let largo = "ñ".repeat(37);
        assert_eq!(
            PasswordPlano::new(largo).err(),
            Some(PasswordPlanoError::MuyLargo)
        );
    }

    #[test]
    fn debug_no_muestra_el_valor() {
        let password = PasswordPlano::new("secreto-de-prueba".to_string()).unwrap();
        assert_eq!(format!("{password:?}"), "PasswordPlano(***)");
    }
}
