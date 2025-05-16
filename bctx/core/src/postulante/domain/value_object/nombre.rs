use crate::postulante::domain::error::nombre::NombreError;
use quizz_common::domain::value_objects::nombre::nombre_regex;

const MAX_TAMANO_NOMBRE: usize = 80;
const MAX_TAMANO_APELLIDO: usize = 80;

/// Representa un nombre completo del postulante
#[derive(Debug)]
pub struct Nombre {
    nombre: String,
    primer_apellido: String,
    segundo_apellido: String,
}

impl Nombre {
    pub fn new(
        nombre: String,
        primer_apellido: String,
        segundo_apellido: String,
    ) -> Result<Self, NombreError> {
        let nombre = Nombre {
            nombre: nombre.trim().to_string(),
            primer_apellido: primer_apellido.trim().to_string(),
            segundo_apellido: segundo_apellido.trim().to_string(),
        };
        nombre.asegurar_nombre_es_correcto()?;
        nombre.asegurar_primer_apellido_es_correcto()?;
        nombre.asegurar_segundo_apellido_es_correcto()?;
        Ok(nombre)
    }

    fn asegurar_nombre_es_correcto(&self) -> Result<(), NombreError> {
        if self.nombre.is_empty() {
            return Err(NombreError::NombreVacio);
        }

        if self.nombre.chars().count() > MAX_TAMANO_NOMBRE {
            return Err(NombreError::NombreExcedeCaracteres);
        }

        if !nombre_regex().is_match(&*self.nombre.to_string()) {
            return Err(NombreError::NombreNoValido);
        }

        Ok(())
    }

    fn asegurar_primer_apellido_es_correcto(&self) -> Result<(), NombreError> {
        if self.primer_apellido.is_empty() {
            return Err(NombreError::ApellidoVacio);
        }

        if self.nombre.chars().count() > MAX_TAMANO_APELLIDO {
            return Err(NombreError::ApellidoExcedeCaracteres);
        }

        if !nombre_regex().is_match(&*self.primer_apellido.to_string()) {
            return Err(NombreError::ApellidoNoValido);
        }

        Ok(())
    }

    fn asegurar_segundo_apellido_es_correcto(&self) -> Result<(), NombreError> {
        if self.segundo_apellido.is_empty() {
            return Err(NombreError::ApellidoVacio);
        }

        if self.nombre.chars().count() > MAX_TAMANO_APELLIDO {
            return Err(NombreError::ApellidoExcedeCaracteres);
        }

        if !nombre_regex().is_match(&*self.segundo_apellido.to_string()) {
            return Err(NombreError::ApellidoNoValido);
        }

        Ok(())
    }

    pub fn nombre(&self) -> &String {
        &self.nombre
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
            self.nombre, self.primer_apellido, self.segundo_apellido
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
        assert_eq!(nombre.nombre(), "John");
        assert_eq!(nombre.primer_apellido(), "Doe");
        assert_eq!(nombre.segundo_apellido(), "Smith");
    }

    #[test]
    fn test_nombre_completo_empty_name() {
        let nombre = Nombre::new("".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::NombreVacio)));
    }

    #[test]
    fn test_nombre_completo_empty_primer_apellido() {
        let nombre = Nombre::new("John".to_string(), "".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::ApellidoVacio)));
    }

    #[test]
    fn test_nombre_completo_empty_segundo_apellido() {
        let nombre = Nombre::new("John".to_string(), "Doe".to_string(), "".to_string());
        assert!(matches!(nombre, Err(NombreError::ApellidoVacio)));
    }

    #[test]
    fn test_nombre_completo_whitespace_name() {
        let nombre = Nombre::new("  ".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::NombreVacio)));
    }

    #[test]
    fn test_nombre_completo_maximo_tamano() {
        let nombre = Nombre::new("María-José Alejandra Guadalupe Rodríguez-Hernández-González-López de la Santísima Trinidad y Todos Los Santos del Monte Carmelo".to_string(), "Doe".to_string(), "Smith".to_string());
        assert!(matches!(nombre, Err(NombreError::NombreExcedeCaracteres)));
    }

    #[test]
    fn test_nombre_incorrecto_caracteres_epeciales() {
        let nombre = Nombre::new(
            "Ju#%ani@to$ Pe\nrez^".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
        );
        assert!(matches!(nombre, Err(NombreError::NombreNoValido)));
    }

    #[test]
    fn test_nombre_incorrecto_numeros() {
        let nombre = Nombre::new(
            "Ju9anito90Perez".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
        );
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
