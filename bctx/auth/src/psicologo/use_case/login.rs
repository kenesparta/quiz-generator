use crate::psicologo::domain::error::psicologo::PsicologoLoginError;
use crate::psicologo::provider::repositorio::{
    RepositorioPsicologoCacheEscritura, RepositorioPsicologoLoginLectura,
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
}

pub struct LoginPsicologoPorDocumento<RepoErr> {
    crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
    repositorio: Box<dyn RepositorioPsicologoLoginLectura<RepoErr>>,
    repositorio_cache: Box<dyn RepositorioPsicologoCacheEscritura<RepoErr>>,
    jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
}

impl<RepoErr> LoginPsicologoPorDocumento<RepoErr> {
    pub fn new(
        crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
        repositorio: Box<dyn RepositorioPsicologoLoginLectura<RepoErr>>,
        repositorio_cache: Box<dyn RepositorioPsicologoCacheEscritura<RepoErr>>,
        jwt: Box<dyn JwtProviderGenerateConRol<RepoErr>>,
    ) -> LoginPsicologoPorDocumento<RepoErr> {
        Self {
            crypto_comparar,
            repositorio,
            repositorio_cache,
            jwt,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, PsicologoLoginError>
    for LoginPsicologoPorDocumento<RepoErr>
where
    PsicologoLoginError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PsicologoLoginError> {
        let psicologo = self
            .repositorio
            .obtener_psicologo_por_documento(in_.documento)
            .await?;

        self.crypto_comparar
            .comparar(in_.password, psicologo.password)
            .await?;

        let jwt_object = self
            .jwt
            .generar_con_rol(psicologo.id, "psicologo".to_string())
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
