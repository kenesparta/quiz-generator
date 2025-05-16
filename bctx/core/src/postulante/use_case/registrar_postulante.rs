use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::service::password::obtener_password_del_documento;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::provider::password::SeguridadPassword;
use crate::postulante::provider::repositorio::RepositorioPostulanteEscritura;
use quizz_common::use_case::CasoDeUso;
use std::str::FromStr;

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

impl<PassErr, RepoErr> CasoDeUso<InputData, OutputData, PostulanteError>
    for RegistrarPostulantePasswordTemporal<PassErr, RepoErr>
where
    PostulanteError: From<PassErr>,
    PostulanteError: From<RepoErr>,
{
    fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let grado_instruccion = GradoInstruccion::from_str(&in_.grado_instruccion)?;
        let genero = Genero::from_str(&in_.genero)?;
        let documento = Documento::new(&in_.documento)?;
        let password = self
            .password_crypto
            .cifrar(obtener_password_del_documento(documento)?.to_string())?;
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
        self.repositorio.registrar_postulante(postulante)?;
        Ok(OutputData {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postulante::domain::error::postulante::RepositorioError;
    use std::error::Error;
    use std::fmt;

    #[derive(Debug)]
    struct MockError;

    impl fmt::Display for MockError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    impl Error for MockError {}

    impl From<MockError> for PostulanteError {
        fn from(_: MockError) -> Self {
            PostulanteError::PostulantePasswordError(
                crate::postulante::domain::error::password::PasswordError::HashNoValido,
            )
        }
    }

    #[derive(Debug)]
    struct MockRepoError;

    impl fmt::Display for MockRepoError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "mock error")
        }
    }

    impl Error for MockRepoError {}

    impl From<MockRepoError> for PostulanteError {
        fn from(_: MockRepoError) -> Self {
            PostulanteError::PostulanteRepositorioError(RepositorioError::PersistenciaNoFinalizada)
        }
    }

    struct MockPasswordCrypto;

    impl SeguridadPassword<MockError> for MockPasswordCrypto {
        fn cifrar(&self, _password: String) -> Result<String, MockError> {
            Ok(
                "$2a$12$w6VMbiyMl1PNVX0R2a8eqOKVwqIl8tMWgxdsaabOmXOuU5N5yYf/m"
                    .parse()
                    .unwrap(),
            )
        }

        fn comparar(&self, _password: String, _hashed: String) -> Result<bool, MockError> {
            Ok(true)
        }
    }

    struct MockRepositorioPostulante {
        id: String,
    }

    impl MockRepositorioPostulante {
        pub fn new(id: String) -> Self {
            MockRepositorioPostulante { id }
        }
    }

    impl RepositorioPostulanteEscritura<MockRepoError> for MockRepositorioPostulante {
        fn registrar_postulante(&self, _postulante: Postulante) -> Result<(), MockRepoError> {
            Ok(())
        }
    }

    #[test]
    fn test_registrar_postulante_exitoso() {
        let use_case = RegistrarPostulantePasswordTemporal::new(
            Box::new(MockPasswordCrypto),
            Box::new(MockRepositorioPostulante::new("abc".to_string())),
        );

        let input = InputData {
            id: "9ee7992b-44dd-426f-aea2-21ca989f9fad".to_string(),
            documento: "12345678".to_string(),
            nombre: "Juan".to_string(),
            apellido_paterno: "Perez".to_string(),
            apellido_materno: "Lopez".to_string(),
            fecha_nacimiento: "1990-01-01".to_string(),
            grado_instruccion: "Superior".to_string(),
            genero: "Masculino".to_string(),
        };

        let result = use_case.ejecutar(input);
        assert!(result.is_ok());
    }

    #[test]
    fn test_registrar_postulante_error_genero() {
        let use_case = RegistrarPostulantePasswordTemporal::new(
            Box::new(MockPasswordCrypto),
            Box::new(MockRepositorioPostulante::new("abc".to_string())),
        );

        let input = InputData {
            id: "12504b7f-ce6a-4549-b9a9-54a2e098abd8".to_string(),
            documento: "12345678".to_string(),
            nombre: "Juan".to_string(),
            apellido_paterno: "Perez".to_string(),
            apellido_materno: "Lopez".to_string(),
            fecha_nacimiento: "1990-01-01".to_string(),
            grado_instruccion: "Superior".to_string(),
            genero: "InvalidGender".to_string(),
        };

        let result = use_case.ejecutar(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_registrar_postulante_error_grado() {
        let use_case = RegistrarPostulantePasswordTemporal::new(
            Box::new(MockPasswordCrypto),
            Box::new(MockRepositorioPostulante::new("abc".to_string())),
        );

        let input = InputData {
            id: "3f2a20bf-f98d-4c86-8259-5cdfebacebca".to_string(),
            documento: "12345678".to_string(),
            nombre: "Juan".to_string(),
            apellido_paterno: "Perez".to_string(),
            apellido_materno: "Lopez".to_string(),
            fecha_nacimiento: "1990-01-01".to_string(),
            grado_instruccion: "InvalidGrado".to_string(),
            genero: "Masculino".to_string(),
        };

        let result = use_case.ejecutar(input);
        assert!(result.is_err());
    }
}
