use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::domain::value_object::evaluacion_estado::EvaluacionEstado;
use crate::evaluacion::value_object::id::EvaluacionID;
use crate::examen::domain::service::lista_examenes::ListaDeExamenes;
use quizz_common::domain::value_objects::estado::EstadoGeneral;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub esta_activo: EstadoGeneral,
    pub estado: EvaluacionEstado,
    pub examenes: ListaDeExamenes,
}

impl Evaluacion {
    pub fn new(id: String, nombre: String, descripcion: String) -> Result<Self, EvaluacionError> {
        if nombre.trim().is_empty() {
            return Err(EvaluacionError::NombreNoValido);
        }

        if descripcion.trim().is_empty() {
            return Err(EvaluacionError::DescripcionNoValida);
        }

        let id = EvaluacionID::new(&id)?;
        let esta_activo = EstadoGeneral::default();
        let estado = EvaluacionEstado::default();

        Ok(Self {
            id,
            nombre,
            descripcion,
            esta_activo,
            estado,
            examenes: ListaDeExamenes::new(Vec::new()),
        })
    }

    /// Pasa de borrador a publicada. Una evaluación se publica una sola vez y solo si tiene
    /// alguna pregunta: si no, los postulantes recibirían un examen vacío.
    ///
    /// # Errors
    ///
    /// `EvaluacionYaFuePublicada` o `EvaluacionSinPreguntas`.
    pub fn publicar(&mut self) -> Result<(), EvaluacionError> {
        if self.esta_publicada() {
            return Err(EvaluacionError::EvaluacionYaFuePublicada);
        }
        let tiene_preguntas = self
            .examenes
            .examenes()
            .iter()
            .any(|examen| !examen.preguntas.preguntas().is_empty());
        if !tiene_preguntas {
            return Err(EvaluacionError::EvaluacionSinPreguntas);
        }
        self.estado = EvaluacionEstado::Publicado;
        Ok(())
    }

    pub fn esta_publicada(&self) -> bool {
        self.estado == EvaluacionEstado::Publicado
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::examen::domain::entity::examen::Examen;
    use crate::examen::domain::value_object::id::ExamenID;
    use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
    use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
    use std::collections::HashMap;

    fn evaluacion_con(preguntas: usize) -> Evaluacion {
        let mut evaluacion = Evaluacion::new(
            "2cf52b7a-0ee3-43a9-9b89-4a8baaa22250".to_string(),
            "Licencia".to_string(),
            "Evaluacion".to_string(),
        )
        .unwrap();
        let preguntas = (0..preguntas)
            .map(|_| {
                PreguntaEntity::new(
                    "¿?".to_string(),
                    "no".to_string(),
                    "libre".to_string(),
                    None,
                    HashMap::new(),
                    HashMap::new(),
                )
                .unwrap()
            })
            .collect();
        evaluacion.examenes = ListaDeExamenes::new(vec![Examen {
            id: ExamenID::new("2fcb7b0d-30e2-4853-afbf-9df79dd83ecb").unwrap(),
            titulo: "T".to_string(),
            descripcion: "D".to_string(),
            instrucciones: "I".to_string(),
            estado: EstadoGeneral::default(),
            preguntas: ListaDePreguntas::new(preguntas),
        }]);
        evaluacion
    }

    #[test]
    fn publicar_una_vez() {
        let mut evaluacion = evaluacion_con(1);
        evaluacion.publicar().unwrap();
        assert!(evaluacion.esta_publicada());
        assert!(matches!(
            evaluacion.publicar(),
            Err(EvaluacionError::EvaluacionYaFuePublicada)
        ));
    }

    #[test]
    fn no_se_publica_sin_preguntas() {
        let mut evaluacion = evaluacion_con(0);
        assert!(matches!(
            evaluacion.publicar(),
            Err(EvaluacionError::EvaluacionSinPreguntas)
        ));
        assert!(!evaluacion.esta_publicada());
    }
}
