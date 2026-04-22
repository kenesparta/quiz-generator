use crate::psicologo::domain::entity::psicologo::Psicologo;
use crate::psicologo::domain::error::psicologo::PsicologoError;
use crate::psicologo::provider::password::SeguridadPasswordPsicologo;
use crate::psicologo::provider::repositorio::RepositorioPsicologoEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

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

pub struct RegistrarPsicologo<PassErr, RepoErr> {
    password_crypto: Box<dyn SeguridadPasswordPsicologo<PassErr>>,
    repositorio: Box<dyn RepositorioPsicologoEscritura<RepoErr>>,
}

impl<PassErr, RepoErr> RegistrarPsicologo<PassErr, RepoErr> {
    pub fn new(
        password_crypto: Box<dyn SeguridadPasswordPsicologo<PassErr>>,
        repositorio: Box<dyn RepositorioPsicologoEscritura<RepoErr>>,
    ) -> RegistrarPsicologo<PassErr, RepoErr> {
        Self {
            password_crypto,
            repositorio,
        }
    }
}

#[async_trait]
impl<PassErr, RepoErr> CasoDeUso<InputData, (), PsicologoError>
    for RegistrarPsicologo<PassErr, RepoErr>
where
    PsicologoError: From<PassErr>,
    PsicologoError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), PsicologoError> {
        let password = self.password_crypto.cifrar(in_.password.clone()).await?;
        let psicologo = Psicologo::new(
            in_.id,
            in_.nombre,
            in_.primer_apellido,
            in_.segundo_apellido,
            in_.documento,
            in_.especialidad,
            in_.colegiatura,
            password,
        )?;
        self.repositorio.registrar_psicologo(psicologo).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockSeguridadPasswordPsicologo {
        _cifrar_result: Result<String, PsicologoError>,
    }

    #[async_trait]
    impl SeguridadPasswordPsicologo<PsicologoError> for MockSeguridadPasswordPsicologo {
        async fn cifrar(&self, _password: String) -> Result<String, PsicologoError> {
            Ok("$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS".to_string())
        }
    }

    struct MockRepositorioPsicologo {
        _psicologo: Mutex<Option<Psicologo>>,
        _result: Result<(), PsicologoError>,
    }

    #[async_trait]
    impl RepositorioPsicologoEscritura<PsicologoError> for MockRepositorioPsicologo {
        async fn registrar_psicologo(&self, _psicologo: Psicologo) -> Result<(), PsicologoError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_registrar_psicologo_success() {
        let password_crypto = Box::new(MockSeguridadPasswordPsicologo {
            _cifrar_result: Ok(
                "$2a$12$/4Ikr2l8lEXk/1iHtiUN7.p/agp333D1PdZjhSzx22PaH0v6rZcZS".to_string(),
            ),
        });

        let repositorio = Box::new(MockRepositorioPsicologo {
            _psicologo: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = RegistrarPsicologo::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                nombre: "Maria".to_string(),
                primer_apellido: "Garcia".to_string(),
                segundo_apellido: "Lopez".to_string(),
                documento: "44556677".to_string(),
                especialidad: "Psicologia Clinica".to_string(),
                colegiatura: "CPP-12345".to_string(),
                password: "mi_password_seguro".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_registrar_psicologo_invalid_documento() {
        let password_crypto = Box::new(MockSeguridadPasswordPsicologo {
            _cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioPsicologo {
            _psicologo: Mutex::new(None),
            _result: Ok(()),
        });

        let use_case = RegistrarPsicologo::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "22d1adea-d489-486b-badf-8e0580ddd0c3".to_string(),
                nombre: "Maria".to_string(),
                primer_apellido: "Garcia".to_string(),
                segundo_apellido: "Lopez".to_string(),
                documento: "12".to_string(),
                especialidad: "Psicologia Clinica".to_string(),
                colegiatura: "CPP-12345".to_string(),
                password: "mi_password_seguro".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
