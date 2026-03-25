use crate::admin::domain::entity::admin::Admin;
use crate::admin::domain::error::admin::AdminError;
use crate::admin::provider::password::SeguridadPasswordAdmin;
use crate::admin::provider::repositorio::RepositorioAdminEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub id: String,
    pub nombre: String,
    pub primer_apellido: String,
    pub segundo_apellido: String,
    pub email: String,
    pub password: String,
}

pub struct RegistrarAdmin<PassErr, RepoErr> {
    password_crypto: Box<dyn SeguridadPasswordAdmin<PassErr>>,
    repositorio: Box<dyn RepositorioAdminEscritura<RepoErr>>,
}

impl<PassErr, RepoErr> RegistrarAdmin<PassErr, RepoErr> {
    pub fn new(
        password_crypto: Box<dyn SeguridadPasswordAdmin<PassErr>>,
        repositorio: Box<dyn RepositorioAdminEscritura<RepoErr>>,
    ) -> RegistrarAdmin<PassErr, RepoErr> {
        Self {
            password_crypto,
            repositorio,
        }
    }
}

#[async_trait]
impl<PassErr, RepoErr> CasoDeUso<InputData, (), AdminError> for RegistrarAdmin<PassErr, RepoErr>
where
    AdminError: From<PassErr>,
    AdminError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), AdminError> {
        let password = self.password_crypto.cifrar(in_.password.clone()).await?;
        let admin = Admin::new(
            in_.id,
            in_.nombre,
            in_.primer_apellido,
            in_.segundo_apellido,
            in_.email,
            password,
        )?;
        self.repositorio.registrar_admin(admin).await?;
        Ok({})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockSeguridadPasswordAdmin {
        _cifrar_result: Result<String, AdminError>,
    }

    #[async_trait]
    impl SeguridadPasswordAdmin<AdminError> for MockSeguridadPasswordAdmin {
        async fn cifrar(&self, _password: String) -> Result<String, AdminError> {
            Ok("$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS".to_string())
        }
    }

    struct MockRepositorioAdmin {
        _admin: Mutex<Option<Admin>>,
        _result: Result<(), AdminError>,
    }

    #[async_trait]
    impl RepositorioAdminEscritura<AdminError> for MockRepositorioAdmin {
        async fn registrar_admin(&self, _admin: Admin) -> Result<(), AdminError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_registrar_admin_success() {
        let password_crypto = Box::new(MockSeguridadPasswordAdmin {
            _cifrar_result: Ok(
                "$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS".to_string(),
            ),
        });

        let repositorio = Box::new(MockRepositorioAdmin {
            _admin: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = RegistrarAdmin::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                nombre: "Carlos".to_string(),
                primer_apellido: "Martinez".to_string(),
                segundo_apellido: "Lopez".to_string(),
                email: "carlos@example.com".to_string(),
                password: "mi_password_seguro".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_registrar_admin_invalid_email() {
        let password_crypto = Box::new(MockSeguridadPasswordAdmin {
            _cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioAdmin {
            _admin: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = RegistrarAdmin::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                nombre: "Carlos".to_string(),
                primer_apellido: "Martinez".to_string(),
                segundo_apellido: "Lopez".to_string(),
                email: "carlos-sin-arroba.com".to_string(),
                password: "mi_password_seguro".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
