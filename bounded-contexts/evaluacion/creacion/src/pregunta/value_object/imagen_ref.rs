use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum ImagenRefError {
    #[error("Referencia de imagen vacía")]
    Vacia,

    #[error("Referencia de imagen inválida: {0}")]
    Invalida(String),
}

/// Value Object que representa una referencia a una imagen.
///
/// Puede ser una URL, un path de archivo, o un identificador único.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ImagenRef {
    valor: String,
}

impl ImagenRef {
    /// Crea una nueva referencia de imagen.
    ///
    /// # Errors
    ///
    /// Retorna error si la referencia está vacía.
    pub fn new(valor: String) -> Result<Self, ImagenRefError> {
        let trimmed = valor.trim();
        if trimmed.is_empty() {
            return Err(ImagenRefError::Vacia);
        }
        Ok(Self {
            valor: trimmed.to_string(),
        })
    }

    #[must_use]
    pub fn valor(&self) -> &str {
        &self.valor
    }

    /// Consume el value object y retorna el string interno.
    #[must_use]
    pub fn into_inner(self) -> String {
        self.valor
    }
}

impl std::fmt::Display for ImagenRef {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.valor)
    }
}

impl AsRef<str> for ImagenRef {
    fn as_ref(&self) -> &str {
        &self.valor
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_creacion_valida() {
        let imagen = ImagenRef::new("https://example.com/image.png".to_string());
        assert!(imagen.is_ok());
        assert_eq!(imagen.unwrap().valor(), "https://example.com/image.png");
    }

    #[test]
    fn test_creacion_vacia() {
        let imagen = ImagenRef::new("".to_string());
        assert!(matches!(imagen, Err(ImagenRefError::Vacia)));
    }

    #[test]
    fn test_creacion_solo_espacios() {
        let imagen = ImagenRef::new("   ".to_string());
        assert!(matches!(imagen, Err(ImagenRefError::Vacia)));
    }

    #[test]
    fn test_trim() {
        let imagen = ImagenRef::new("  path/to/image.jpg  ".to_string()).unwrap();
        assert_eq!(imagen.valor(), "path/to/image.jpg");
    }

    #[test]
    fn test_display() {
        let imagen = ImagenRef::new("img.png".to_string()).unwrap();
        assert_eq!(format!("{}", imagen), "img.png");
    }
}
