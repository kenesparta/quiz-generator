use crate::postulante::domain::error::postulante::PostulanteLoginError;
use crate::postulante::provider::repositorio::{
    RepositorioPostulanteCacheEscritura, RepositorioPostulanteLoginLectura,
};
use async_trait::async_trait;
use quizz_common::provider::jwt::JwtProviderGenerate;
use quizz_common::provider::seguridad::{SeguridadCifrar, SeguridadComparar};
use quizz_common::use_case::CasoDeUso;

pub struct InputData {
    pub documento: String,
    pub password: String,
}

pub struct OutputData {
    pub jwt_value: String,
    pub expiration: u64,
}

pub struct LoginPostulantePorDocumento<RepoErr> {
    crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
    repositorio: Box<dyn RepositorioPostulanteLoginLectura<RepoErr>>,
    repositorio_cache: Box<dyn RepositorioPostulanteCacheEscritura<RepoErr>>,
    jwt: Box<dyn JwtProviderGenerate<RepoErr>>,
}

impl<RepoErr> LoginPostulantePorDocumento<RepoErr> {
    pub fn new(
        crypto_comparar: Box<dyn SeguridadComparar<RepoErr>>,
        repositorio: Box<dyn RepositorioPostulanteLoginLectura<RepoErr>>,
        repositorio_cache: Box<dyn RepositorioPostulanteCacheEscritura<RepoErr>>,
        jwt: Box<dyn JwtProviderGenerate<RepoErr>>,
    ) -> LoginPostulantePorDocumento<RepoErr> {
        Self {
            crypto_comparar,
            repositorio,
            repositorio_cache,
            jwt,
        }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, OutputData, PostulanteLoginError>
    for LoginPostulantePorDocumento<RepoErr>
where
    PostulanteLoginError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteLoginError> {
        let postulante = self
            .repositorio
            .obtener_postulante_por_documento(in_.documento)
            .await?;

        self.crypto_comparar
            .comparar(in_.password, postulante.password)
            .await?;

        let jwt_object = self.jwt.generar(postulante.id).await?;

        self.repositorio_cache
            .guardar_token(jwt_object.clone())
            .await?;

        Ok(OutputData {
            jwt_value: jwt_object.value,
            expiration: jwt_object.expiration,
        })
    }
}
