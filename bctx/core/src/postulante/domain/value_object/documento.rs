use crate::postulante::domain::error::documento::DocumentoError;
use std::fmt;

const MIN_DOCUMENT_LENGTH: usize = 4;
/// Un DNI tiene 8 dígitos y un carné de extranjería hasta 12 caracteres; 20 deja margen sin
/// aceptar cadenas arbitrariamente largas.
const MAX_DOCUMENT_LENGTH: usize = 20;

/// El número de documento del postulante (p. ej., identificación nacional, pasaporte). El tipo
/// y formato específicos de este número dependerán de los requisitos de la aplicación.
/// Esta propiedad también debe ser único en el contexto de la aplicación.
///
/// Es un dato personal: `Debug` no lo muestra, para que un `{:?}` no lo lleve a los logs.
pub struct Documento(String);

impl fmt::Debug for Documento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Documento(***)")
    }
}

impl fmt::Display for Documento {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Documento {
    pub fn new(value: &str) -> Result<Self, DocumentoError> {
        let document = Documento(value.trim().to_string());
        document.asegurar_documento_es_valido()?;
        Ok(document)
    }

    /// Solo letras y dígitos ASCII, entre 4 y 20 caracteres. Al ser ASCII, cada carácter es un
    /// byte y recortar el texto nunca puede caer a mitad de un carácter (antes "ñ123" hacía
    /// panic al registrar al postulante, R-029). También evita que "1234-5678" y "12345678"
    /// cuenten como documentos distintos.
    pub fn asegurar_documento_es_valido(&self) -> Result<(), DocumentoError> {
        if self.0.is_empty() || !self.0.bytes().all(|b| b.is_ascii_alphanumeric()) {
            return Err(DocumentoError::DocumentoNoValido);
        }

        if !(MIN_DOCUMENT_LENGTH..=MAX_DOCUMENT_LENGTH).contains(&self.0.len()) {
            return Err(DocumentoError::TamanioDocumentoNoPermitido);
        }

        Ok(())
    }

    pub fn obtener_ultimos_cuatro_caracteres(&self) -> Result<String, DocumentoError> {
        self.asegurar_documento_es_valido()?;
        let last_four = &self.0[self.0.len() - MIN_DOCUMENT_LENGTH..];
        Ok(last_four.to_string())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

#[cfg(test)]
mod test_documento {
    use super::*;

    #[test]
    fn test_crear_documento_valido() {
        let documento = Documento::new("12345678").unwrap();
        assert_eq!(documento.value(), "12345678");
    }

    #[test]
    fn test_crear_documento_vacio() {
        let result = Documento::new("");
        assert!(matches!(result, Err(DocumentoError::DocumentoNoValido)));
    }

    #[test]
    fn test_crear_documento_solo_espacios() {
        let result = Documento::new("   ");
        assert!(matches!(result, Err(DocumentoError::DocumentoNoValido)));
    }

    #[test]
    fn test_validar_documento() {
        let documento = Documento("12345678".to_string());
        assert!(documento.asegurar_documento_es_valido().is_ok());
    }

    #[test]
    fn test_get_last_four_characters_success() {
        let documento = Documento::new("12345678").unwrap();
        assert_eq!(
            documento.obtener_ultimos_cuatro_caracteres().unwrap(),
            "5678"
        );
    }

    #[test]
    fn test_get_last_four_characters_exact_length() {
        let documento = Documento::new("12345678").unwrap();
        assert_eq!(
            documento.obtener_ultimos_cuatro_caracteres().unwrap(),
            "5678"
        );
    }

    #[test]
    fn test_get_last_four_characters_invalid_length() {
        let documento = Documento::new("123");
        assert!(matches!(
            documento,
            Err(DocumentoError::TamanioDocumentoNoPermitido)
        ));
    }

    #[test]
    fn caracteres_no_ascii_o_separadores_no_son_validos() {
        for documento in ["ñ123", "1234-5678", "12 345 678", "x&y=z", "1€€4"] {
            assert!(
                matches!(
                    Documento::new(documento),
                    Err(DocumentoError::DocumentoNoValido)
                ),
                "{documento}"
            );
        }
    }

    #[test]
    fn la_longitud_tiene_minimo_y_maximo() {
        assert!(Documento::new("1234").is_ok());
        assert!(Documento::new(&"1".repeat(MAX_DOCUMENT_LENGTH)).is_ok());
        assert!(matches!(
            Documento::new(&"1".repeat(MAX_DOCUMENT_LENGTH + 1)),
            Err(DocumentoError::TamanioDocumentoNoPermitido)
        ));
    }

    #[test]
    fn los_ultimos_cuatro_nunca_hacen_panic() {
        let documento = Documento::new("CE0012345").unwrap();
        assert_eq!(
            documento.obtener_ultimos_cuatro_caracteres().unwrap(),
            "2345"
        );
    }
}
