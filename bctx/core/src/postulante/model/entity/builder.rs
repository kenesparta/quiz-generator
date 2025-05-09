use crate::postulante::model::value_object::documento::Documento;
use crate::postulante::model::value_object::genero::Genero;
use crate::postulante::model::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::model::value_object::id::PostulanteID;
use crate::postulante::model::value_object::nombre::Nombre;
use crate::postulante::model::value_object::password::Password;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;

#[derive(Debug)]
pub struct PostulanteBuilder {
    id: Option<PostulanteID>,
    documento: Option<Documento>,
    nombre_completo: Option<Nombre>,
    fecha_nacimiento: Option<FechaNacimiento>,
    grado_instruccion: Option<GradoInstruccion>,
    genero: Option<Genero>,
    password: Option<Password>,
}
