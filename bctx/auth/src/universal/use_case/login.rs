use crate::universal::domain::error::login_universal::LoginUniversalError;
use crate::universal::provider::repositorio::{
    RepositorioLoginUniversalCacheEscritura, RepositorioLoginUniversalLectura,
};
use async_trait::async_trait;
use quizz_common::provider::jwt::JwtProviderGenerateConRol;
use quizz_common::provider::seguridad::SeguridadComparar;
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub documento: String,
    pub password: String,
}

pub struct OutputData {
    pub jwt_value: String,
    pub expiration: u64,
    pub rol: String,
}

pub struct LoginUniversal<RepoErr> {
    crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
    repositorio: Box<dyn RepositorioLoginUniversalLectura<RepoErr>>,
    repositorio_cache: Box<dyn RepositorioLoginUniversalCacheEscritura<RepoErr>>,
    jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
}

impl<RepoErr> LoginUniversal<RepoErr> {
    pub fn new(
        crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
        repositorio: Box<dyn RepositorioLoginUniversalLectura<RepoErr>>,
        repositorio_cache: Box<dyn RepositorioLoginUniversalCacheEscritura<RepoErr>>,
        jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
    ) -> LoginUniversal<RepoErr> {
        Self {
            crypto_comparar,
            repositorio,
            repositorio_cache,
            jwt,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, LoginUniversalError> for LoginUniversal<RepoErr>
where
    LoginUniversalError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, LoginUniversalError> {
        let usuario = self.repositorio.buscar_por_documento(in_.documento).await?;

        self.crypto_comparar
            .comparar(in_.password, usuario.password)
            .await?;

        let jwt_object = self
            .jwt
            .generar_con_rol(usuario.id, usuario.rol.clone())
            .await?;

        self.repositorio_cache
            .guardar_token(jwt_object.clone())
            .await?;

        Ok(OutputData {
            jwt_value: jwt_object.value,
            expiration: jwt_object.expiration,
            rol: usuario.rol,
        })
    }
}
