use crate::admin::domain::entity::admin::Admin;
use crate::admin::domain::error::admin::AdminError;
use crate::admin::provider::repositorio::RepositorioAdminEscritura;
use async_trait::async_trait;
use quizz_common::domain::value_objects::password_plano::PasswordPlano;
use quizz_common::provider::seguridad::Cifrador;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub documento: String,
    pub password: String,
}

/// Registra un admin. La contraseña se valida en texto plano antes de calcular el hash, y solo
/// el hash llega a la entidad y al repositorio.
pub struct RegistrarAdmin<RepoErr> {
    password_crypto: Box<dyn Cifrador>,
    repositorio: Box<dyn RepositorioAdminEscritura<RepoErr>>,
}

impl<RepoErr> RegistrarAdmin<RepoErr> {
    pub fn new(
        password_crypto: Box<dyn Cifrador>,
        repositorio: Box<dyn RepositorioAdminEscritura<RepoErr>>,
    ) -> RegistrarAdmin<RepoErr> {
        Self {
            password_crypto,
            repositorio,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), AdminError> for RegistrarAdmin<RepoErr>
where
    AdminError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), AdminError> {
        let password = PasswordPlano::new(in_.password)?;
        let hash = self.password_crypto.cifrar(password.into_inner()).await?;
        let admin = Admin::new(
            in_.id,
            in_.nombre,
            in_.primer_apellido,
            in_.segundo_apellido,
            in_.documento,
            hash,
        )?;
        self.repositorio.registrar_admin(admin).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizz_common::domain::value_objects::password_plano::PasswordPlanoError;
    use quizz_common::provider::seguridad::CifradoError;
    use std::sync::{Arc, Mutex};

    const HASH: &str = "$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS";

    /// Registra las contraseñas que recibe y devuelve siempre el mismo hash.
    #[derive(Clone, Default)]
    struct CifradorFalso(Arc<Mutex<Vec<String>>>);

    #[async_trait]
    impl Cifrador for CifradorFalso {
        async fn cifrar(&self, password: String) -> Result<String, CifradoError> {
            self.0.lock().unwrap().push(password);
            Ok(HASH.to_string())
        }

        async fn verificar(&self, _: String, _: String) -> Result<bool, CifradoError> {
            Ok(false)
        }
    }

    /// Guarda los admins registrados.
    #[derive(Clone, Default)]
    struct RepositorioFalso(Arc<Mutex<Vec<Admin>>>);

    #[async_trait]
    impl RepositorioAdminEscritura<AdminError> for RepositorioFalso {
        async fn registrar_admin(&self, admin: Admin) -> Result<(), AdminError> {
            self.0.lock().unwrap().push(admin);
            Ok(())
        }
    }

    fn entrada(password: &str) -> InputData {
        InputData {
            id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
            nombre: "Carlos".to_string(),
            primer_apellido: "Martinez".to_string(),
            segundo_apellido: "Lopez".to_string(),
            documento: "12345678".to_string(),
            password: password.to_string(),
        }
    }

    #[tokio::test]
    async fn guarda_el_hash_y_no_la_contraseña() {
        let (cifrador, repositorio) = (CifradorFalso::default(), RepositorioFalso::default());
        RegistrarAdmin::new(Box::new(cifrador.clone()), Box::new(repositorio.clone()))
            .ejecutar(entrada("una-clave-segura"))
            .await
            .unwrap();

        assert_eq!(*cifrador.0.lock().unwrap(), ["una-clave-segura"]);
        let guardados = repositorio.0.lock().unwrap();
        assert_eq!(guardados[0].password.as_deref(), Some(HASH));
    }

    #[tokio::test]
    async fn una_contraseña_vacia_o_corta_se_rechaza_sin_calcular_el_hash() {
        for (password, esperado) in [
            ("", PasswordPlanoError::Vacio),
            ("corta", PasswordPlanoError::MuyCorto),
        ] {
            let (cifrador, repositorio) = (CifradorFalso::default(), RepositorioFalso::default());
            let resultado =
                RegistrarAdmin::new(Box::new(cifrador.clone()), Box::new(repositorio.clone()))
                    .ejecutar(entrada(password))
                    .await;

            assert!(
                matches!(&resultado, Err(AdminError::PasswordNoValido(e)) if *e == esperado),
                "{password:?}"
            );
            assert!(cifrador.0.lock().unwrap().is_empty());
            assert!(repositorio.0.lock().unwrap().is_empty());
        }
    }

    #[tokio::test]
    async fn un_documento_no_valido_no_se_guarda() {
        let repositorio = RepositorioFalso::default();
        let mut datos = entrada("una-clave-segura");
        datos.documento = "123".to_string();
        let resultado = RegistrarAdmin::new(
            Box::new(CifradorFalso::default()),
            Box::new(repositorio.clone()),
        )
        .ejecutar(datos)
        .await;

        assert!(matches!(resultado, Err(AdminError::DocumentoNoValido(_))));
        assert!(repositorio.0.lock().unwrap().is_empty());
    }
}
