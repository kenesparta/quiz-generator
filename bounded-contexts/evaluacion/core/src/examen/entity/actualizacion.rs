use evaluacion_common::SimpleName;

use super::examen::Examen;

/// Operaciones de actualización del examen.
impl Examen {
    /// Actualiza el título del examen.
    pub fn actualizar_titulo(&mut self, titulo: SimpleName) {
        self.titulo = titulo;
    }

    /// Actualiza la descripción del examen.
    pub fn actualizar_descripcion(&mut self, descripcion: Option<SimpleName>) {
        self.descripcion = descripcion;
    }

    /// Actualiza las instrucciones del examen.
    pub fn actualizar_instrucciones(&mut self, instrucciones: Option<SimpleName>) {
        self.instrucciones = instrucciones;
    }
}

#[cfg(test)]
mod tests {
    use super::super::examen::TITULO_CONFIG;
    use super::*;

    fn crear_examen_ejemplo() -> Examen {
        Examen::new(
            "Examen de Prueba".to_string(),
            Some("Descripción del examen".to_string()),
            Some("Lea cuidadosamente".to_string()),
        )
        .unwrap()
    }

    #[test]
    fn test_actualizar_titulo() {
        let mut examen = crear_examen_ejemplo();
        let nuevo_titulo =
            SimpleName::with_config("Nuevo Título".to_string(), TITULO_CONFIG).unwrap();

        examen.actualizar_titulo(nuevo_titulo);

        assert_eq!(examen.titulo().as_str(), "Nuevo Título");
    }

    #[test]
    fn test_actualizar_descripcion() {
        let mut examen = crear_examen_ejemplo();
        let nueva_descripcion = SimpleName::new("Nueva descripción".to_string()).unwrap();

        examen.actualizar_descripcion(Some(nueva_descripcion));

        assert_eq!(examen.descripcion().unwrap().as_str(), "Nueva descripción");
    }

    #[test]
    fn test_actualizar_descripcion_a_none() {
        let mut examen = crear_examen_ejemplo();

        examen.actualizar_descripcion(None);

        assert!(examen.descripcion().is_none());
    }

    #[test]
    fn test_actualizar_instrucciones() {
        let mut examen = crear_examen_ejemplo();
        let nuevas_instrucciones = SimpleName::new("Nuevas instrucciones".to_string()).unwrap();

        examen.actualizar_instrucciones(Some(nuevas_instrucciones));

        assert_eq!(
            examen.instrucciones().unwrap().as_str(),
            "Nuevas instrucciones"
        );
    }

    #[test]
    fn test_actualizar_instrucciones_a_none() {
        let mut examen = crear_examen_ejemplo();

        examen.actualizar_instrucciones(None);

        assert!(examen.instrucciones().is_none());
    }
}
