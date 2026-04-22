use crate::admin::domain::error::admin::AdminLoginError;
use crate::admin::provider::repositorio::{
    RepositorioAdminCacheEscritura, RepositorioAdminLoginLectura,
};
use crate::autorizacion::domain::value_object::rol::Rol;
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
}

pub struct LoginAdminPorDocumento<RepoErr> {
    crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
    repositorio: Box<dyn RepositorioAdminLoginLectura<RepoErr>>,
    repositorio_cache: Box<dyn RepositorioAdminCacheEscritura<RepoErr>>,
    jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
}

impl<RepoErr> LoginAdminPorDocumento<RepoErr> {
    pub fn new(
        crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
        repositorio: Box<dyn RepositorioAdminLoginLectura<RepoErr>>,
        repositorio_cache: Box<dyn RepositorioAdminCacheEscritura<RepoErr>>,
        jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
    ) -> LoginAdminPorDocumento<RepoErr> {
        Self {
            crypto_comparar,
            repositorio,
            repositorio_cache,
            jwt,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, AdminLoginError> for LoginAdminPorDocumento<RepoErr>
where
    AdminLoginError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, AdminLoginError> {
        let admin = self
            .repositorio
            .obtener_admin_por_documento(in_.documento)
            .await?;

        self.crypto_comparar
            .comparar(in_.password, admin.password)
            .await?;

        let jwt_object = self
            .jwt
            .generar_con_rol(admin.id, Rol::Admin.to_string())
            .await?;

        self.repositorio_cache
            .guardar_token(jwt_object.clone())
            .await?;

        Ok(OutputData {
            jwt_value: jwt_object.value,
            expiration: jwt_object.expiration,
        })
    }
}
