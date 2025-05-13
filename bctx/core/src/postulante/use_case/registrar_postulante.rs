use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
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

impl<E> CasoDeUso<InputData, OutputData, PostulanteError>
    for RegistrarPostulantePasswordTemporal<E>
{
    fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let grado_instruccion = GradoInstruccion::from_str(&in_.grado_instruccion)?;
        let genero = Genero::from_str(&in_.genero)?;
        let pass = self.password_crypto.cifrar(in_.documento.to_string())?;
        let postulante = Postulante::new(
            in_.id,
            in_.documento,
            in_.nombre,
            in_.apellido_paterno,
            in_.apellido_materno,
            in_.fecha_nacimiento,
            grado_instruccion,
            genero,
            "$2a$12$b0a7aabc6PcLyAMKifb3pOCSwi8zgqf0ylujb8DgF3I1r.xn.Mrn2".to_string(),
        )?;
        Ok(OutputData {})
    }
}
