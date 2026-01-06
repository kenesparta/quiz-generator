use common::Id;

use crate::Pregunta;
use crate::examen::Examen;
use crate::examen::error::ExamenError;

/// Operaciones relacionadas con las preguntas del examen.
impl Examen {
    /// Agrega una pregunta al examen.
    pub fn agregar_pregunta(&mut self, pregunta: Pregunta) {
        self.preguntas.push(pregunta);
    }

    /// Agrega múltiples preguntas al examen.
    pub fn agregar_preguntas(&mut self, preguntas: impl IntoIterator<Item = Pregunta>) {
        self.preguntas.extend(preguntas);
    }

    /// Elimina una pregunta por su ID.
    ///
    /// # Errors
    ///
    /// Retorna `ExamenError::PreguntaNoEncontrada` si no existe una pregunta con ese ID.
    pub fn eliminar_pregunta(&mut self, pregunta_id: Id) -> Result<Pregunta, ExamenError> {
        let posicion = self
            .preguntas
            .iter()
            .position(|p| p.id() == pregunta_id)
            .ok_or(ExamenError::PreguntaNoEncontrada(pregunta_id))?;

        Ok(self.preguntas.remove(posicion))
    }

    /// Elimina una pregunta por su índice.
    ///
    /// # Errors
    ///
    /// Retorna `ExamenError::IndiceFueraDeRango` si el índice está fuera de rango.
    pub fn eliminar_pregunta_por_indice(&mut self, indice: usize) -> Result<Pregunta, ExamenError> {
        if indice >= self.preguntas.len() {
            return Err(ExamenError::IndiceFueraDeRango {
                indice,
                maximo: self.preguntas.len().saturating_sub(1),
            });
        }
        Ok(self.preguntas.remove(indice))
    }

    /// Obtiene una pregunta por su ID.
    #[must_use]
    pub fn obtener_pregunta(&self, pregunta_id: Id) -> Option<&Pregunta> {
        self.preguntas().iter().find(|p| p.id() == pregunta_id)
    }

    /// Obtiene una pregunta por su índice.
    ///
    /// # Errors
    ///
    /// Retorna `ExamenError::IndiceFueraDeRango` si el índice está fuera de rango.
    pub fn obtener_pregunta_por_indice(&self, indice: usize) -> Result<&Pregunta, ExamenError> {
        self.preguntas()
            .get(indice)
            .ok_or(ExamenError::IndiceFueraDeRango {
                indice,
                maximo: self.preguntas().len().saturating_sub(1),
            })
    }

    /// Reordena las preguntas según los ID proporcionados.
    ///
    /// Las preguntas que no estén en la lista de ID se colocan al final.
    pub fn reordenar_preguntas(&mut self, orden: &[Id]) {
        self.preguntas.sort_by_key(|p| {
            orden
                .iter()
                .position(|&id| id == p.id())
                .unwrap_or(usize::MAX)
        });
    }

    /// Limpia todas las preguntas del examen.
    pub fn limpiar_preguntas(&mut self) {
        self.preguntas.clear();
    }
}

#[cfg(test)]
mod tests {
    use common::SimpleName;

    use super::*;
    use crate::pregunta::*;

    fn crear_examen_ejemplo() -> Examen {
        Examen::new(
            "Examen de Prueba".to_string(),
            Some("Descripción del examen".to_string()),
            Some("Lea cuidadosamente".to_string()),
        )
        .unwrap()
    }

    fn crear_pregunta_ejemplo(contenido: &str) -> Pregunta {
        let contenido = SimpleName::new(contenido.to_string()).unwrap();
        Pregunta::Libre(PreguntaLibre::new(contenido, None, Etiqueta::No))
    }

    #[test]
    fn test_agregar_pregunta() {
        let mut examen = crear_examen_ejemplo();
        let pregunta = crear_pregunta_ejemplo("¿Pregunta 1?");

        examen.agregar_pregunta(pregunta);

        assert_eq!(examen.cantidad_preguntas(), 1);
        assert!(!examen.esta_vacio());
    }

    #[test]
    fn test_agregar_multiples_preguntas() {
        let mut examen = crear_examen_ejemplo();
        let preguntas = vec![
            crear_pregunta_ejemplo("¿Pregunta 1?"),
            crear_pregunta_ejemplo("¿Pregunta 2?"),
            crear_pregunta_ejemplo("¿Pregunta 3?"),
        ];

        examen.agregar_preguntas(preguntas);

        assert_eq!(examen.cantidad_preguntas(), 3);
    }

    #[test]
    fn test_eliminar_pregunta_por_id() {
        let mut examen = crear_examen_ejemplo();
        let pregunta = crear_pregunta_ejemplo("¿Pregunta 1?");
        let pregunta_id = pregunta.id();

        examen.agregar_pregunta(pregunta);
        assert_eq!(examen.cantidad_preguntas(), 1);

        let resultado = examen.eliminar_pregunta(pregunta_id);
        assert!(resultado.is_ok());
        assert!(examen.esta_vacio());
    }

    #[test]
    fn test_eliminar_pregunta_inexistente() {
        let mut examen = crear_examen_ejemplo();
        let id_inexistente = Id::new();

        let resultado = examen.eliminar_pregunta(id_inexistente);
        assert!(matches!(
            resultado,
            Err(ExamenError::PreguntaNoEncontrada(_))
        ));
    }

    #[test]
    fn test_eliminar_pregunta_por_indice() {
        let mut examen = crear_examen_ejemplo();
        examen.agregar_pregunta(crear_pregunta_ejemplo("¿Pregunta 1?"));
        examen.agregar_pregunta(crear_pregunta_ejemplo("¿Pregunta 2?"));

        let resultado = examen.eliminar_pregunta_por_indice(0);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap().contenido().as_str(), "¿Pregunta 1?");
        assert_eq!(examen.cantidad_preguntas(), 1);
    }

    #[test]
    fn test_eliminar_pregunta_indice_fuera_de_rango() {
        let mut examen = crear_examen_ejemplo();

        let resultado = examen.eliminar_pregunta_por_indice(10);
        assert!(matches!(
            resultado,
            Err(ExamenError::IndiceFueraDeRango { .. })
        ));
    }

    #[test]
    fn test_obtener_pregunta() {
        let mut examen = crear_examen_ejemplo();
        let pregunta = crear_pregunta_ejemplo("¿Pregunta 1?");
        let pregunta_id = pregunta.id();

        examen.agregar_pregunta(pregunta);

        let encontrada = examen.obtener_pregunta(pregunta_id);
        assert!(encontrada.is_some());
        assert_eq!(encontrada.unwrap().contenido().as_str(), "¿Pregunta 1?");
    }

    #[test]
    fn test_obtener_pregunta_por_indice() {
        let mut examen = crear_examen_ejemplo();
        examen.agregar_pregunta(crear_pregunta_ejemplo("¿Pregunta 1?"));

        let resultado = examen.obtener_pregunta_por_indice(0);
        assert!(resultado.is_ok());
        assert_eq!(resultado.unwrap().contenido().as_str(), "¿Pregunta 1?");
    }

    #[test]
    fn test_obtener_pregunta_por_indice_fuera_de_rango() {
        let examen = crear_examen_ejemplo();

        let resultado = examen.obtener_pregunta_por_indice(10);
        assert!(matches!(
            resultado,
            Err(ExamenError::IndiceFueraDeRango { .. })
        ));
    }

    #[test]
    fn test_limpiar_preguntas() {
        let mut examen = crear_examen_ejemplo();
        examen.agregar_pregunta(crear_pregunta_ejemplo("¿Pregunta 1?"));
        examen.agregar_pregunta(crear_pregunta_ejemplo("¿Pregunta 2?"));

        examen.limpiar_preguntas();

        assert!(examen.esta_vacio());
    }

    #[test]
    fn test_reordenar_preguntas() {
        let mut examen = crear_examen_ejemplo();
        let p1 = crear_pregunta_ejemplo("Primera");
        let p2 = crear_pregunta_ejemplo("Segunda");
        let p3 = crear_pregunta_ejemplo("Tercera");

        let id1 = p1.id();
        let id2 = p2.id();
        let id3 = p3.id();

        examen.agregar_preguntas(vec![p1, p2, p3]);

        // Reordenar: Tercera, Primera, Segunda
        examen.reordenar_preguntas(&[id3, id1, id2]);

        assert_eq!(examen.preguntas()[0].contenido().as_str(), "Tercera");
        assert_eq!(examen.preguntas()[1].contenido().as_str(), "Primera");
        assert_eq!(examen.preguntas()[2].contenido().as_str(), "Segunda");
    }

    #[test]
    fn test_agregar_preguntas_diferentes_tipos() {
        let mut examen = crear_examen_ejemplo();

        // 1. Pregunta Libre
        let pregunta_libre = Pregunta::Libre(PreguntaLibre::new(
            SimpleName::new("¿Cuál es su experiencia laboral?".to_string()).unwrap(),
            None,
            Etiqueta::No,
        ));

        // 2. Pregunta Alternativa Única
        let alternativas_unica = AlternativasMultiples::new(vec![
            Alternativa::new(AlternativaClave::A, "Opción A".to_string()),
            Alternativa::new(AlternativaClave::B, "Opción B".to_string()),
            Alternativa::new(AlternativaClave::C, "Opción C".to_string()),
        ])
        .unwrap();
        let puntaje_unica =
            PuntajeAlternativaUnica::new(AlternativaClave::B, Puntaje::uno()).unwrap();
        let pregunta_unica = Pregunta::AlternativaUnica(PreguntaAlternativaUnica::new(
            SimpleName::new("¿Cuál es la respuesta correcta?".to_string()).unwrap(),
            None,
            Etiqueta::No,
            alternativas_unica,
            puntaje_unica,
        ));

        // 3. Pregunta Alternativa Con Peso (Likert)
        let alternativas_peso = AlternativasMultiples::new(vec![
            Alternativa::new(AlternativaClave::A, "Muy en desacuerdo".to_string()),
            Alternativa::new(AlternativaClave::B, "En desacuerdo".to_string()),
            Alternativa::new(AlternativaClave::C, "Neutral".to_string()),
            Alternativa::new(AlternativaClave::D, "De acuerdo".to_string()),
            Alternativa::new(AlternativaClave::E, "Muy de acuerdo".to_string()),
        ])
        .unwrap();
        let puntaje_peso = PuntajeConPeso::new(
            [
                (AlternativaClave::A, Puntaje::new(0.0).unwrap()),
                (AlternativaClave::B, Puntaje::new(1.0).unwrap()),
                (AlternativaClave::C, Puntaje::new(2.0).unwrap()),
                (AlternativaClave::D, Puntaje::new(3.0).unwrap()),
                (AlternativaClave::E, Puntaje::new(4.0).unwrap()),
            ]
            .into_iter()
            .collect(),
        )
        .unwrap();
        let pregunta_peso = Pregunta::AlternativaConPeso(PreguntaAlternativaConPeso::new(
            SimpleName::new("¿Qué tan satisfecho está con el servicio?".to_string()).unwrap(),
            None,
            Etiqueta::No,
            alternativas_peso,
            puntaje_peso,
        ));

        // 4. Pregunta Sí/No
        let alternativas_si_no = AlternativasSiNo::default_texts();
        let puntaje_si_no = PuntajeSiNo::solo_si(Puntaje::uno());
        let pregunta_si_no = Pregunta::SiNo(PreguntaSiNo::new(
            SimpleName::new("¿Le gusta trabajar en equipo?".to_string()).unwrap(),
            None,
            Etiqueta::No,
            alternativas_si_no,
            puntaje_si_no,
        ));

        // 5. Pregunta Sola Respuesta
        let puntaje_sola = PuntajeSolaRespuesta::new("París".to_string(), Puntaje::uno());
        let pregunta_sola = Pregunta::SolaRespuesta(PreguntaSolaRespuesta::new(
            SimpleName::new("¿Cuál es la capital de Francia?".to_string()).unwrap(),
            None,
            Etiqueta::No,
            puntaje_sola,
        ));

        // Agregar todas las preguntas al examen
        examen.agregar_preguntas(vec![
            pregunta_libre,
            pregunta_unica,
            pregunta_peso,
            pregunta_si_no,
            pregunta_sola,
        ]);

        // Verificar cantidad
        assert_eq!(examen.cantidad_preguntas(), 5);

        // Verificar tipos
        assert!(examen.preguntas()[0].es_libre());
        assert!(examen.preguntas()[1].es_alternativa_unica());
        assert!(examen.preguntas()[2].es_alternativa_con_peso());
        assert!(examen.preguntas()[3].es_si_no());
        assert!(examen.preguntas()[4].es_sola_respuesta());

        // Verificar contenidos
        assert_eq!(
            examen.preguntas()[0].contenido().as_str(),
            "¿Cuál es su experiencia laboral?"
        );
        assert_eq!(
            examen.preguntas()[1].contenido().as_str(),
            "¿Cuál es la respuesta correcta?"
        );
        assert_eq!(
            examen.preguntas()[2].contenido().as_str(),
            "¿Qué tan satisfecho está con el servicio?"
        );
        assert_eq!(
            examen.preguntas()[3].contenido().as_str(),
            "¿Le gusta trabajar en equipo?"
        );
        assert_eq!(
            examen.preguntas()[4].contenido().as_str(),
            "¿Cuál es la capital de Francia?"
        );

        // Verificar tipo_nombre
        assert_eq!(examen.preguntas()[0].tipo_nombre(), "libre");
        assert_eq!(examen.preguntas()[1].tipo_nombre(), "alternativa_unica");
        assert_eq!(examen.preguntas()[2].tipo_nombre(), "alternativa_peso");
        assert_eq!(examen.preguntas()[3].tipo_nombre(), "si_o_no");
        assert_eq!(examen.preguntas()[4].tipo_nombre(), "sola_respuesta");
    }
}
