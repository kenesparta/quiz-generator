use crate::psicologo::domain::entity::psicologo::Psicologo;
use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::provider::repositorio::RepositorioPsicologoEscritura;
use quizz_common::domain::value_objects::password_plano::PasswordPlano;
use quizz_common::provider::seguridad::Cifrador;

pub struct InputData {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub especialidad: String,
    pub colegiatura: String,
    pub password: String,
}

/// Registra un psicólogo. La contraseña se valida en texto plano antes de calcular el hash, y
/// solo el hash llega a la entidad y al repositorio.
pub struct RegistrarPsicologo<C, R> {
    password_crypto: C,
    repositorio: R,
}

impl<C: Cifrador, R: RepositorioPsicologoEscritura> RegistrarPsicologo<C, R> {
    pub fn new(password_crypto: C, repositorio: R) -> Self {
        Self {
            password_crypto,
            repositorio,
        }
    }

    pub async fn ejecutar(&self, in_: InputData) -> Result<(), PsicologoError> {
        let password = PasswordPlano::new(in_.password)?;
        let hash = self.password_crypto.cifrar(password.into_inner()).await?;
        let psicologo = Psicologo::new(
            in_.id,
            in_.nombre,
            in_.primer_apellido,
            in_.segundo_apellido,
            in_.documento,
            in_.especialidad,
            in_.colegiatura,
            hash,
        )?;
        self.repositorio.registrar_psicologo(psicologo).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::psicologo::domain::value_object::id::PsicologoID;
    use quizz_common::domain::value_objects::password_plano::PasswordPlanoError;
    use quizz_common::provider::seguridad::CifradoError;
    use std::sync::{Arc, Mutex};

    const HASH: &str = "$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS";

    /// Registra las contraseñas que recibe y devuelve siempre el mismo hash.
    #[derive(Clone, Default)]
    struct CifradorFalso(Arc<Mutex<Vec<String>>>);

    impl Cifrador for CifradorFalso {
        async fn cifrar(&self, password: String) -> Result<String, CifradoError> {
            self.0.lock().unwrap().push(password);
            Ok(HASH.to_string())
        }

        async fn verificar(&self, _: String, _: String) -> Result<bool, CifradoError> {
            Ok(false)
        }

        async fn simular_verificacion(&self, _: String) {}
    }

    /// Guarda los psicólogos registrados.
    #[derive(Clone, Default)]
    struct RepositorioFalso(Arc<Mutex<Vec<Psicologo>>>);

    impl RepositorioPsicologoEscritura for RepositorioFalso {
        async fn registrar_psicologo(&self, psicologo: Psicologo) -> Result<(), PsicologoError> {
            self.0.lock().unwrap().push(psicologo);
            Ok(())
        }

        async fn eliminar_psicologo(&self, _: PsicologoID) -> Result<(), PsicologoError> {
            Ok(())
        }
    }

    fn entrada(password: &str) -> InputData {
        InputData {
            id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
            nombre: "Maria".to_string(),
            primer_apellido: "Garcia".to_string(),
            segundo_apellido: "Lopez".to_string(),
            documento: "12345678".to_string(),
            especialidad: "Psicologia Clinica".to_string(),
            colegiatura: "CPsP-12345".to_string(),
            password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn guarda_el_hash_y_no_la_contraseña() {
        let (cifrador, repositorio) = (CifradorFalso::default(), RepositorioFalso::default());
        RegistrarPsicologo::new(cifrador.clone(), repositorio.clone())
            .ejecutar(entrada("una-clave-segura"))
            .await
            .unwrap();

        assert_eq!(*cifrador.0.lock().unwrap(), ["una-clave-segura"]);
        let guardados = repositorio.0.lock().unwrap();
        assert_eq!(guardados[0].password.as_deref(), Some(HASH));
        assert_eq!(guardados[0].colegiatura, "CPsP-12345");
    }

    #[tokio::test]
    async fn una_contraseña_no_valida_se_rechaza_sin_calcular_el_hash() {
        for (password, esperado) in [
            ("", PasswordPlanoError::Vacio),
            ("corta", PasswordPlanoError::MuyCorto),
            (&"x".repeat(73), PasswordPlanoError::MuyLargo),
        ] {
            let (cifrador, repositorio) = (CifradorFalso::default(), RepositorioFalso::default());
            let resultado = RegistrarPsicologo::new(cifrador.clone(), repositorio.clone())
                .ejecutar(entrada(password))
                .await;

            assert!(
                matches!(&resultado, Err(PsicologoError::PasswordNoValido(e)) if *e == esperado),
                "{password:?}"
            );
            assert!(cifrador.0.lock().unwrap().is_empty());
            assert!(repositorio.0.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn sin_colegiatura_no_se_guarda() {
        let repositorio = RepositorioFalso::default();
        let mut datos = entrada("una-clave-segura");
        datos.colegiatura = " ".to_string();
        let resultado = RegistrarPsicologo::new(CifradorFalso::default(), repositorio.clone())
            .ejecutar(datos)
            .await;

        assert!(matches!(resultado, Err(PsicologoError::ColegiaturaVacia)));
        assert!(repositorio.0.lock().unwrap().is_empty());
    }
}
