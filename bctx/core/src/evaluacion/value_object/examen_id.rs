use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use quizz_common::domain::value_objects::id::ID;

/// Los exámenes a asociar a una evaluación: al menos uno, todos con un id válido y sin
/// repetidos.
pub struct ExamenIDs {
    pub examen_ids: Vec<ID>,
}

impl ExamenIDs {
    /// # Errors
    ///
    /// `SinExamenes` si la lista está vacía y `ExamenIdNoValido` con el primer id mal formado
    /// (antes se descartaban en silencio y la petición respondía 200 sin asociar nada).
    pub fn new(examen_ids: Vec<String>) -> Result<Self, EvaluacionError> {
        if examen_ids.is_empty() {
            return Err(EvaluacionError::SinExamenes);
        }
        let mut ids: Vec<ID> = Vec::with_capacity(examen_ids.len());
        for texto in examen_ids {
            let id =
                ID::new(&texto).map_err(|_| EvaluacionError::ExamenIdNoValido(texto.clone()))?;
            if !ids.contains(&id) {
                ids.push(id);
            }
        }
        Ok(Self { examen_ids: ids })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const UNO: &str = "2fcb7b0d-30e2-4853-afbf-9df79dd83ecb";
    const OTRO: &str = "19573e4f-321d-41ad-a8a9-3807c6fd3d65";

    #[test]
    fn un_id_mal_formado_es_un_error_y_no_se_descarta() {
        let resultado = ExamenIDs::new(vec![UNO.to_string(), "no-es-un-uuid".to_string()]);
        assert!(matches!(
            resultado,
            Err(EvaluacionError::ExamenIdNoValido(id)) if id == "no-es-un-uuid"
        ));
    }

    #[test]
    fn una_lista_vacia_es_un_error() {
        assert!(matches!(
            ExamenIDs::new(Vec::new()),
            Err(EvaluacionError::SinExamenes)
        ));
    }

    #[test]
    fn los_repetidos_cuentan_una_vez() {
        let ids = ExamenIDs::new(vec![UNO.to_string(), OTRO.to_string(), UNO.to_string()]).unwrap();
        assert_eq!(ids.examen_ids.len(), 2);
    }
}
