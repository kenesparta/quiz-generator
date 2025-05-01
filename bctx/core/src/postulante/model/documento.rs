use crate::postulante::model::errors::DocumentoError;

/// El número de documento del postulante (p. ej., identificación nacional, pasaporte). El tipo
/// y formato específicos de este número dependerán de los requisitos de la aplicación.
/// Esta propiedad tambien debe ser único en el contexto de la aplicación.
pub struct Documento(String);

impl Documento {
    pub fn new(value: String) -> Result<Self, DocumentoError> {
        let document = Documento(value);
        document.asegurar_documento_es_valido()?;
        Ok(document)
    }

    pub fn asegurar_documento_es_valido(&self) -> Result<(), DocumentoError> {
        if self.0.trim().is_empty() {
            return Err(DocumentoError::DocumentoNoValido);
        }

        Ok(())
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
        let documento = Documento::new("12345678".to_string()).unwrap();
        assert_eq!(documento.value(), "12345678");
    }

    #[test]
    fn test_crear_documento_vacio() {
        let result = Documento::new("".to_string());
        assert!(matches!(result, Err(DocumentoError::DocumentoNoValido)));
    }

    #[test]
    fn test_crear_documento_solo_espacios() {
        let result = Documento::new("   ".to_string());
        assert!(matches!(result, Err(DocumentoError::DocumentoNoValido)));
    }

    #[test]
    fn test_validar_documento() {
        let documento = Documento("12345678".to_string());
        assert!(documento.asegurar_documento_es_valido().is_ok());
    }
}
