use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::psicologo::provider::repositorio::RepositorioPsicologoEscritura;
use crate::psicologo::provider::sesion::SesionPsicologo;

pub struct InputData {
    pub id: String,
}

/// Elimina definitivamente a un psicólogo y cierra su sesión, para que su token deje de
/// autenticar en ese momento y no cuando expire. Las revisiones que calificó conservan su id
/// en `revisado_por`, pero su nombre y su colegiatura ya no se pueden leer.
///
/// Va en tres pasos: marcarlo (ya no puede iniciar sesión), cerrar su sesión y borrarlo.
/// - Cerrar la sesión después de marcarlo deja fuera a un inicio de sesión simultáneo: el
///   login vuelve a buscar la cuenta tras abrir su sesión, así que o ve la marca, o abrió la
///   sesión antes de que se cerrara.
/// - Si la sesión no se puede cerrar, el psicólogo queda marcado y no se borra: repetir la
///   eliminación la completa (borrarlo antes dejaría un token vigente y nada que reintentar).
///
/// # Errors
///
/// `PsicologoIdError` si el id no es un UUID, `RegistroNoEncontrado` si no hay un psicólogo
/// con ese id (y entonces no se toca ninguna sesión) y `SesionNoCerrada` si la eliminación
/// quedó a medias.
pub struct EliminarPsicologo<R, S> {
    repositorio: R,
    sesion: S,
}

impl<R: RepositorioPsicologoEscritura, S: SesionPsicologo> EliminarPsicologo<R, S> {
    pub fn new(repositorio: R, sesion: S) -> Self {
        Self {
            repositorio,
            sesion,
        }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), PsicologoError> {
        let id = PsicologoID::new(&in_.id)?;
        self.repositorio.marcar_eliminado(id).await?;
        self.sesion.cerrar(id).await?;
        self.repositorio.eliminar_psicologo(id).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::psicologo::domain::entity::psicologo::Psicologo;
    use crate::psicologo::domain::error::psicologo::RepositorioError;
    use quizz_common::domain::value_objects::id::IdError;
    use std::sync::atomic::{AtomicBool, Ordering};
    use std::sync::{Arc, Mutex};

    const ID: &str = "22d1adea-d489-486b-badf-8e0580ddd0c3";

    /// Repositorio y sesiones a la vez: guarda un psicólogo y anota cada paso con su id.
    #[derive(Clone)]
    struct Falso {
        guardado: Arc<Mutex<Option<String>>>,
        pasos: Arc<Mutex<Vec<String>>>,
        sesiones_caidas: Arc<AtomicBool>,
    }

    impl Falso {
        fn con_psicologo() -> Self {
            Self {
                guardado: Arc::new(Mutex::new(Some(ID.to_string()))),
                pasos: Arc::default(),
                sesiones_caidas: Arc::default(),
            }
        }

        fn anotar(&self, paso: &str, id: PsicologoID) {
            self.pasos.lock().unwrap().push(format!("{paso} {id}"));
        }

        fn pasos(&self) -> Vec<String> {
            self.pasos.lock().unwrap().clone()
        }

        async fn eliminar(&self, id: &str) -> Result<(), PsicologoError> {
            EliminarPsicologo::new(self.clone(), self.clone())
                .ejecutar(InputData { id: id.to_string() })
                .await
        }
    }

    impl RepositorioPsicologoEscritura for Falso {
        async fn registrar_psicologo(&self, _: Psicologo) -> Result<(), PsicologoError> {
            Ok(())
        }

        async fn marcar_eliminado(&self, id: PsicologoID) -> Result<(), PsicologoError> {
            self.anotar("marcar", id);
            if self.guardado.lock().unwrap().as_deref() != Some(&id.to_string()) {
                return Err(PsicologoError::PsicologoRepositorioError(
                    RepositorioError::RegistroNoEncontrado,
                ));
            }
            Ok(())
        }

        async fn eliminar_psicologo(&self, id: PsicologoID) -> Result<(), PsicologoError> {
            self.anotar("eliminar", id);
            self.guardado.lock().unwrap().take();
            Ok(())
        }
    }

    impl SesionPsicologo for Falso {
        async fn cerrar(&self, id: PsicologoID) -> Result<(), PsicologoError> {
            self.anotar("cerrar", id);
            if self.sesiones_caidas.load(Ordering::SeqCst) {
                return Err(PsicologoError::SesionNoCerrada);
            }
            Ok(())
        }
    }

    #[tokio::test]
    async fn marca_cierra_la_sesion_y_borra_en_ese_orden_con_el_id_canonico() {
        let falso = Falso::con_psicologo();
        falso.eliminar(&ID.to_uppercase()).await.unwrap();

        assert_eq!(
            falso.pasos(),
            [
                format!("marcar {ID}"),
                format!("cerrar {ID}"),
                format!("eliminar {ID}")
            ]
        );
        assert!(falso.guardado.lock().unwrap().is_none());
    }

    #[tokio::test]
    async fn si_la_sesion_no_se_cierra_no_se_borra_y_reintentar_lo_completa() {
        let falso = Falso::con_psicologo();
        falso.sesiones_caidas.store(true, Ordering::SeqCst);

        let resultado = falso.eliminar(ID).await;

        assert!(matches!(resultado, Err(PsicologoError::SesionNoCerrada)));
        assert_eq!(
            falso.pasos(),
            [format!("marcar {ID}"), format!("cerrar {ID}")]
        );
        assert!(falso.guardado.lock().unwrap().is_some());

        falso.sesiones_caidas.store(false, Ordering::SeqCst);
        falso.eliminar(ID).await.unwrap();

        assert_eq!(falso.pasos().last(), Some(&format!("eliminar {ID}")));
        assert!(falso.guardado.lock().unwrap().is_none());
    }

    #[tokio::test]
    async fn un_psicologo_que_no_existe_no_cierra_ninguna_sesion() {
        let falso = Falso::con_psicologo();
        let otro = "872c8c81-9fab-494a-9267-799876261bcb";

        let resultado = falso.eliminar(otro).await;

        assert!(matches!(
            resultado,
            Err(PsicologoError::PsicologoRepositorioError(
                RepositorioError::RegistroNoEncontrado
            ))
        ));
        assert_eq!(falso.pasos(), [format!("marcar {otro}")]);
    }

    #[tokio::test]
    async fn un_id_no_valido_es_un_error_de_id() {
        let falso = Falso::con_psicologo();

        let resultado = falso.eliminar("no-es-un-uuid").await;

        assert!(matches!(
            resultado,
            Err(PsicologoError::PsicologoIdError(IdError::FormatoNoValido))
        ));
        assert!(falso.pasos().is_empty());
    }
}
