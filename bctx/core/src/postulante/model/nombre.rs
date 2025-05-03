use crate::postulante::model::errors::NombreError;

#[cfg(test)]
mod tests {
    use super::*;
}

/// Representa un nombre completo del postulante
pub struct Nombre {
    /// Todos los nombres del postulante.
    name: String,
    primer_apellido: String,
    segundo_apellido: String,
}

impl Nombre {
    pub fn new(
        name: String,
        primer_apellido: String,
        segundo_apellido: String,
    ) -> Result<Self, NombreError> {
        let full_name = Nombre {
            name,
            primer_apellido,
            segundo_apellido,
        };
        full_name.asegurar_nombre_es_correcto()?;
        full_name.asegurar_primer_apellido_es_correcto()?;
        full_name.asegurar_segundo_apellido_es_correcto()?;
        Ok(full_name)
    }

    fn asegurar_nombre_es_correcto(&self) -> Result<(), NombreError> {
        if self.name.trim().is_empty() {
            return Err(NombreError::NombreNoValido);
        }

        Ok(())
    }

    fn asegurar_primer_apellido_es_correcto(&self) -> Result<(), NombreError> {
        if self.primer_apellido.trim().is_empty() {
            return Err(NombreError::ApellidosNoValidos);
        }

        Ok(())
    }

    fn asegurar_segundo_apellido_es_correcto(&self) -> Result<(), NombreError> {
        if self.segundo_apellido.trim().is_empty() {
            return Err(NombreError::ApellidosNoValidos);
        }

        Ok(())
    }

    pub fn name(&self) -> &String {
        &self.name
    }

    pub fn primer_apellido(&self) -> &String {
        &self.primer_apellido
    }

    pub fn segundo_apellido(&self) -> &String {
        &self.segundo_apellido
    }

    pub fn nombre_completo(&self) -> String {
        format!(
            "{} {} {}",
            self.name, self.primer_apellido, self.segundo_apellido
        )
    }
}

#[cfg(test)]
mod test_nombre_completo {
    use super::*;

    #[test]
    fn test_nombre_completo_creation_success() {
        let nombre = Nombre::new("John".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(nombre.is_ok());
        let nombre = nombre.unwrap();
        assert_eq!(nombre.name(), "John");
        assert_eq!(nombre.primer_apellido(), "Doe");
        assert_eq!(nombre.segundo_apellido(), "Smith");
    }

    #[test]
    fn test_nombre_completo_empty_name() {
        let nombre = Nombre::new("".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::NombreNoValido)));
    }

    #[test]
    fn test_nombre_completo_empty_primer_apellido() {
        let nombre = Nombre::new("John".to_string(), "".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::ApellidosNoValidos)));
    }

    #[test]
    fn test_nombre_completo_empty_segundo_apellido() {
        let nombre = Nombre::new("John".to_string(), "Doe".to_string(), "".to_string());
        assert!(matches!(nombre, Err(NombreError::ApellidosNoValidos)));
    }

    #[test]
    fn test_nombre_completo_whitespace_name() {
        let nombre = Nombre::new(" ".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::NombreNoValido)));
    }

    #[test]
    fn test_nombre_completo_returns_correctly_formatted_string() {
        let nombre =
            Nombre::new("Juan".to_string(), "Pérez".to_string(), "Gómez".to_string()).unwrap();

        let resultado = nombre.nombre_completo();

        assert_eq!(resultado, "Juan Pérez Gómez");
    }

    #[test]
    fn test_nombre_completo_with_multi_word_name() {
        let nombre = Nombre::new(
            "María José".to_string(),
            "Rodríguez".to_string(),
            "López".to_string(),
        )
        .unwrap();

        let resultado = nombre.nombre_completo();
        assert_eq!(resultado, "María José Rodríguez López");
    }

    #[test]
    fn test_nombre_completo_preserves_capitalizations() {
        let nombre = Nombre::new(
            "Carlos".to_string(),
            "de la Cruz".to_string(),
            "MARTÍNEZ".to_string(),
        )
        .unwrap();

        let resultado = nombre.nombre_completo();
        assert_eq!(resultado, "Carlos de la Cruz MARTÍNEZ");
    }
}
