use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::service::password::obtener_password_del_documento;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::provider::password::SeguridadPassword;
use crate::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use std::sync::Mutex;

    struct MockSeguridadPassword {
        cifrar_result: Result<String, PostulanteError>,
    }

    #[async_trait]
    impl SeguridadPassword<PostulanteError> for MockSeguridadPassword {
        async fn cifrar(&self, _password: String) -> Result<String, PostulanteError> {
            Ok("".to_string())
        }

        async fn comparar(
            &self,
            _password: String,
            _hashed: String,
        ) -> Result<bool, PostulanteError> {
            unimplemented!()
        }
    }

    struct MockRepositorioPostulante {
        postulante: Mutex<Option<Postulante>>,
        result: Result<(), PostulanteError>,
    }

    #[async_trait]
    impl RepositorioPostulanteEscritura<PostulanteError> for MockRepositorioPostulante {
        async fn registrar_postulante(
            &self,
            postulante: Postulante,
        ) -> Result<(), PostulanteError> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_registrar_postulante_success() {
        let password_crypto = Box::new(MockSeguridadPassword {
            cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioPostulante {
            postulante: Mutex::new(None),
            result: Ok(()),
        });

        let use_case = RegistrarPostulantePasswordTemporal::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "1".to_string(),
                documento: "12345678".to_string(),
                nombre: "John".to_string(),
                apellido_paterno: "Doe".to_string(),
                apellido_materno: "Smith".to_string(),
                fecha_nacimiento: "1990-01-01".to_string(),
                grado_instruccion: "SECUNDARIA".to_string(),
                genero: "MASCULINO".to_string(),
            })
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_registrar_postulante_invalid_documento() {
        let password_crypto = Box::new(MockSeguridadPassword {
            cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioPostulante {
            postulante: Mutex::new(None),
            result: Ok(()),
        });

        let use_case = RegistrarPostulantePasswordTemporal::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "1".to_string(),
                documento: "123".to_string(), // Invalid document
                nombre: "John".to_string(),
                apellido_paterno: "Doe".to_string(),
                apellido_materno: "Smith".to_string(),
                fecha_nacimiento: "1990-01-01".to_string(),
                grado_instruccion: "SECUNDARIA".to_string(),
                genero: "MASCULINO".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registrar_postulante_invalid_genero() {
        let password_crypto = Box::new(MockSeguridadPassword {
            cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioPostulante {
            postulante: Mutex::new(None),
            result: Ok(()),
        });

        let use_case = RegistrarPostulantePasswordTemporal::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "1".to_string(),
                documento: "12345678".to_string(),
                nombre: "John".to_string(),
                apellido_paterno: "Doe".to_string(),
                apellido_materno: "Smith".to_string(),
                fecha_nacimiento: "1990-01-01".to_string(),
                grado_instruccion: "SECUNDARIA".to_string(),
                genero: "INVALID".to_string(),
            })
            .await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_registrar_postulante_invalid_grado() {
        let password_crypto = Box::new(MockSeguridadPassword {
            cifrar_result: Ok("hashed_password".to_string()),
        });

        let repositorio = Box::new(MockRepositorioPostulante {
            postulante: Mutex::new(None),
            result: Ok(()),
        });

        let use_case = RegistrarPostulantePasswordTemporal::new(password_crypto, repositorio);

        let result = use_case
            .ejecutar(InputData {
                id: "1".to_string(),
                documento: "12345678".to_string(),
                nombre: "John".to_string(),
                apellido_paterno: "Doe".to_string(),
                apellido_materno: "Smith".to_string(),
                fecha_nacimiento: "1990-01-01".to_string(),
                grado_instruccion: "INVALID".to_string(),
                genero: "MASCULINO".to_string(),
            })
            .await;

        assert!(result.is_err());
    }
}

pub struct InputData {
    pub id: String,
    pub documento: String,
    pub nombre: String,
    pub apellido_paterno: String,
    pub apellido_materno: String,
    pub fecha_nacimiento: String,
    pub grado_instruccion: String,
    pub genero: String,
}

pub struct OutputData {}

pub struct RegistrarPostulantePasswordTemporal<PassErr, RepoErr> {
    password_crypto: Box<dyn SeguridadPassword<PassErr>>,
    repositorio: Box<dyn RepositorioPostulanteEscritura<RepoErr>>,
}

impl<PassErr, RepoErr> RegistrarPostulantePasswordTemporal<PassErr, RepoErr> {
    pub fn new(
        password_crypto: Box<dyn SeguridadPassword<PassErr>>,
        repositorio: Box<dyn RepositorioPostulanteEscritura<RepoErr>>,
    ) -> RegistrarPostulantePasswordTemporal<PassErr, RepoErr> {
        Self {
            password_crypto,
            repositorio,
        }
    }
}

#[async_trait]
impl<PassErr, RepoErr> CasoDeUso<InputData, OutputData, PostulanteError>
    for RegistrarPostulantePasswordTemporal<PassErr, RepoErr>
where
    PostulanteError: From<PassErr>,
    PostulanteError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let grado_instruccion = GradoInstruccion::from_str(&in_.grado_instruccion)?;
        let genero = Genero::from_str(&in_.genero)?;
        let documento = Documento::new(&in_.documento)?;
        let password = self
            .password_crypto
            .cifrar(obtener_password_del_documento(documento)?.to_string())
            .await?;
        let postulante = Postulante::new(
            in_.id,
            in_.documento,
            in_.nombre,
            in_.apellido_paterno,
            in_.apellido_materno,
            in_.fecha_nacimiento,
            grado_instruccion,
            genero,
            password,
        )?;
        self.repositorio.registrar_postulante(postulante).await?;
        Ok(OutputData {})
    }
}
