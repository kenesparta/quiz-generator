use crate::postulante::model::documento::Documento;
use crate::postulante::model::genero::Genero;
use crate::postulante::model::grado_instruccion::GradoInstruccion;
use crate::postulante::model::id::PostulanteID;
use crate::postulante::model::nombre::Nombre;
use crate::postulante::model::password::Password;

/// Representa al postulante para obtener la licencia de conducir.
/// Este postulante es creado para poder realizar el examen
pub struct Postulante {
    id: PostulanteID,
    numero_documento: Documento,
    nombre_completo: Nombre,
    grado_instruccion: GradoInstruccion,
    genero: Genero,
    fecha_nacimiento: String,

    /// Este password es temporal y se genera en el momento de crear el postulante
    password: Password,
}
