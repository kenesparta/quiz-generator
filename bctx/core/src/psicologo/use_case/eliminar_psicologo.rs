use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::psicologo::provider::repositorio::RepositorioPsicologoEscritura;

pub struct InputData {
    pub id: String,
}

/// Elimina definitivamente a un psicólogo. Las revisiones que calificó conservan su id en
/// `revisado_por`, pero su nombre y su colegiatura ya no se pueden leer.
///
/// Devuelve el id en su forma canónica, que es el sujeto de los tokens del psicólogo: con él
/// quien llama cierra su sesión, que vive en otro contexto (`quizz-auth`).
///
/// # Errors
///
/// `PsicologoIdError` si el id no es un UUID (sin llegar al repositorio) y
/// `RegistroNoEncontrado` si no hay un psicólogo con ese id.
pub struct EliminarPsicologo<R> {
    repositorio: R,
}

impl<R: RepositorioPsicologoEscritura> EliminarPsicologo<R> {
    pub fn new(repositorio: R) -> Self {
        Self { repositorio }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<PsicologoID, PsicologoError> {
        let id = PsicologoID::new(&in_.id)?;
        self.repositorio.eliminar_psicologo(id).await?;
        Ok(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::psicologo::domain::entity::psicologo::Psicologo;
    use crate::psicologo::domain::error::psicologo::RepositorioError;
    use quizz_common::domain::value_objects::id::IdError;
    use std::sync::{Arc, Mutex};

    const ID: &str = "22d1adea-d489-486b-badf-8e0580ddd0c3";

    /// Los ids de los psicólogos guardados.
    #[derive(Clone, Default)]
    struct RepositorioFalso(Arc<Mutex<Vec<String>>>);

    impl RepositorioFalso {
        fn con(id: &str) -> Self {
            Self(Arc::new(Mutex::new(vec![id.to_string()])))
        }
    }

    impl RepositorioPsicologoEscritura for RepositorioFalso {
        async fn registrar_psicologo(&self, _: Psicologo) -> Result<(), PsicologoError> {
            Ok(())
        }

        async fn eliminar_psicologo(&self, id: PsicologoID) -> Result<(), PsicologoError> {
            let mut guardados = self.0.lock().unwrap();
            let antes = guardados.len();
            guardados.retain(|guardado| *guardado != id.to_string());
            if guardados.len() == antes {
                return Err(PsicologoError::PsicologoRepositorioError(
                    RepositorioError::RegistroNoEncontrado,
                ));
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn borra_por_el_id_canonico_y_lo_devuelve() {
        let repositorio = RepositorioFalso::con(ID);
        let eliminado = EliminarPsicologo::new(repositorio.clone())
            .ejecutar(InputData {
                id: ID.to_uppercase(),
            })
            .await
            .unwrap();

        assert_eq!(eliminado.to_string(), ID);
        assert!(repositorio.0.lock().unwrap().is_empty());
    }

    #[tokio::test]
    async fn un_psicologo_que_no_existe_es_registro_no_encontrado() {
        let repositorio = RepositorioFalso::con(ID);
        let resultado = EliminarPsicologo::new(repositorio.clone())
            .ejecutar(InputData {
                id: "872c8c81-9fab-494a-9267-799876261bcb".to_string(),
            })
            .await;

        assert!(matches!(
            resultado,
            Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroNoEncontrado
            ))
        ));
        assert_eq!(*repositorio.0.lock().unwrap(), [ID]);
    }

    #[tokio::test]
    async fn un_id_no_valido_es_un_error_de_id() {
        let repositorio = RepositorioFalso::con(ID);
        let resultado = EliminarPsicologo::new(repositorio.clone())
            .ejecutar(InputData {
                id: "no-es-un-uuid".to_string(),
            })
            .await;

        assert!(matches!(
            resultado,
            Err(PsicologoError::PsicologoIdError(IdError::FormatoNoValido))
        ));
        assert_eq!(*repositorio.0.lock().unwrap(), [ID]);
    }
}
