use crate::universal::domain::error::login_universal::LoginUniversalError;
use crate::universal::provider::repositorio::RepositorioLoginUniversalCacheBorrado;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub sujeto_id: String,
}

pub struct Logout<RepoErr> {
    repositorio_cache: Box<dyn RepositorioLoginUniversalCacheBorrado<RepoErr>>,
}

impl<RepoErr> Logout<RepoErr> {
    pub fn new(
        repositorio_cache: Box<dyn RepositorioLoginUniversalCacheBorrado<RepoErr>>,
    ) -> Logout<RepoErr> {
        Self { repositorio_cache }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), LoginUniversalError> for Logout<RepoErr>
where
    LoginUniversalError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), LoginUniversalError> {
        self.repositorio_cache.borrar_token(in_.sujeto_id).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct MockRepo {
        borrados: Mutex<Vec<String>>,
    }

    #[async_trait]
    impl RepositorioLoginUniversalCacheBorrado<LoginUniversalError> for MockRepo {
        async fn borrar_token(&self, sujeto_id: String) -> Result<(), LoginUniversalError> {
            self.borrados.lock().unwrap().push(sujeto_id);
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_logout_borra_token_redis() {
        let repo = Box::new(MockRepo {
            borrados: Mutex::new(Vec::new()),
        });
        let use_case = Logout::new(repo);

        let resultado = use_case
            .ejecutar(InputData {
                sujeto_id: "usr-123".to_string(),
            })
            .await;

        assert!(resultado.is_ok());
    }
}
