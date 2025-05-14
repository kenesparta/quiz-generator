use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::service::password::obtener_password_del_documento;
use crate::postulante::domain::value_object::documento::Documento;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::provider::password::PasswordCrypto;
use quizz_common::use_case::CasoDeUso;
use std::str::FromStr;

struct InputData {
    id: String,
    documento: String,
    nombre: String,
    apellido_paterno: String,
    apellido_materno: String,
    fecha_nacimiento: String,
    grado_instruccion: String,
    genero: String,
}

struct OutputData {}

pub struct RegistrarPostulantePasswordTemporal<E> {
    password_crypto: Box<dyn PasswordCrypto<E>>,
}

impl<E> RegistrarPostulantePasswordTemporal<E> {
    pub fn new(
        password_crypto: Box<dyn PasswordCrypto<E>>,
    ) -> RegistrarPostulantePasswordTemporal<E> {
        Self { password_crypto }
    }
}

impl<E> CasoDeUso<InputData, OutputData, PostulanteError> for RegistrarPostulantePasswordTemporal<E>
where
    PostulanteError: From<E>,
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
        Ok(OutputData {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;
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

    struct MockPasswordCrypto;

    impl PasswordCrypto<MockError> for MockPasswordCrypto {
        fn cifrar(&self, password: String) -> Result<String, MockError> {
            Ok(
                "$2a$12$w6VMbiyMl1PNVX0R2a8eqOKVwqIl8tMWgxdsaabOmXOuU5N5yYf/m"
                    .parse()
                    .unwrap(),
            )
        }

        fn comparar(&self, password: String, hashed: String) -> Result<bool, MockError> {
            todo!()
        }
    }

    #[test]
    fn test_registrar_postulante_exitoso() {
        let use_case = RegistrarPostulantePasswordTemporal::new(Box::new(MockPasswordCrypto));

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
        let use_case = RegistrarPostulantePasswordTemporal::new(Box::new(MockPasswordCrypto));

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
        let use_case = RegistrarPostulantePasswordTemporal::new(Box::new(MockPasswordCrypto));

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
