use crate::usuario::domain::entity::usuario::Usuario;
use crate::usuario::domain::error::usuario::UsuarioError;
use crate::usuario::provider::repositorio::RepositorioUsuarioEscritura;
use async_trait::async_trait;
use quizz_common::provider::seguridad::SeguridadCifrar;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub id: String,
    pub nombre: String,
    pub email: String,
    pub password: String,
    pub rol: String,
}

pub struct RegistrarUsuario<RepoErr> {
    crypto: Box<dyn SeguridadCifrar<RepoErr>>,
    repositorio: Box<dyn RepositorioUsuarioEscritura<RepoErr>>,
}

impl<RepoErr> RegistrarUsuario<RepoErr> {
    pub fn new(
        crypto: Box<dyn SeguridadCifrar<RepoErr>>,
        repositorio: Box<dyn RepositorioUsuarioEscritura<RepoErr>>,
    ) -> Self {
        Self {
            crypto,
            repositorio,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), UsuarioError> for RegistrarUsuario<RepoErr>
where
    UsuarioError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), UsuarioError> {
        let hashed_password = self
            .crypto
            .cifrar(in_.password.clone())
            .await
            .map_err(|_| UsuarioError::ErrorCifrado)?;

        let usuario = Usuario::new(in_.id, in_.nombre, in_.email, hashed_password, in_.rol)?;

        self.repositorio
            .registrar_usuario(usuario)
            .await
            .map_err(|_| UsuarioError::RepositorioError)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockCrypto;

    #[async_trait]
    impl SeguridadCifrar<UsuarioError> for MockCrypto {
        async fn cifrar(&self, _password: String) -> Result<String, UsuarioError> {
            Ok("$2a$12$hashedpassword".to_string())
        }
    }

    struct MockRepositorio {
        _usuario: Mutex<Option<Usuario>>,
    }

    #[async_trait]
    impl RepositorioUsuarioEscritura<UsuarioError> for MockRepositorio {
        async fn registrar_usuario(&self, usuario: Usuario) -> Result<(), UsuarioError> {
            let mut guard = self._usuario.lock().unwrap();
            *guard = Some(usuario);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_registrar_usuario_exitoso() {
        let use_case = RegistrarUsuario::new(
            Box::new(MockCrypto),
            Box::new(MockRepositorio {
                _usuario: Mutex::new(None),
            }),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "abc-123".to_string(),
                nombre: "Juan Perez".to_string(),
                email: "juan@example.com".to_string(),
                password: "mi_password_seguro".to_string(),
                rol: "psicologo".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_registrar_usuario_email_invalido() {
        let use_case = RegistrarUsuario::new(
            Box::new(MockCrypto),
            Box::new(MockRepositorio {
                _usuario: Mutex::new(None),
            }),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "abc-123".to_string(),
                nombre: "Juan Perez".to_string(),
                email: "no-email".to_string(),
                password: "pass".to_string(),
                rol: "admin".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registrar_usuario_rol_invalido() {
        let use_case = RegistrarUsuario::new(
            Box::new(MockCrypto),
            Box::new(MockRepositorio {
                _usuario: Mutex::new(None),
            }),
        );

        let result = use_case
            .ejecutar(InputData {
                id: "abc-123".to_string(),
                nombre: "Juan Perez".to_string(),
                email: "juan@example.com".to_string(),
                password: "pass".to_string(),
                rol: "superadmin".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}
