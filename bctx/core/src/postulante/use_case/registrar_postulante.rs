use crate::postulante::domain::entity::postulante::Postulante;
use crate::postulante::domain::error::postulante::PostulanteError;
use crate::postulante::domain::value_object::genero::Genero;
use crate::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
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

pub struct RegistrarPostulante {}

impl CasoDeUso<InputData, OutputData, PostulanteError> for RegistrarPostulante {
    fn ejecutar(&self, in_: InputData) -> Result<OutputData, PostulanteError> {
        let grado_instruccion = GradoInstruccion::from_str(&in_.grado_instruccion)?;
        let genero = Genero::from_str(&in_.genero)?;
        let postulante = Postulante::new(
            in_.id,
            in_.documento,
            in_.nombre,
            in_.apellido_paterno,
            in_.apellido_materno,
            in_.fecha_nacimiento,
            grado_instruccion,
            genero,
        )?;
        Ok(OutputData {})
    }
}
