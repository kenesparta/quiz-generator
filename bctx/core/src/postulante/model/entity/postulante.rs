use crate::postulante::model::error::postulante::PostulanteError;
use crate::postulante::model::value_object::documento::Documento;
use crate::postulante::model::value_object::genero::Genero;
use crate::postulante::model::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::model::value_object::id::PostulanteID;
use crate::postulante::model::value_object::nombre::Nombre;
use crate::postulante::model::value_object::password::Password;

/// Representa al postulante para obtener la licencia de conducir.
/// Este postulante es creado para poder realizar el examen
pub struct Postulante {
    id: PostulanteID,
    numero_documento: Documento,
    nombre_completo: Nombre,
    fecha_nacimiento: String,
    grado_instruccion: GradoInstruccion,
    genero: Genero,
    password: String,
}

impl Postulante {
    pub fn new(
        id: PostulanteID,
        numero_documento: Documento,
        nombre_completo: Nombre,
        fecha_nacimiento: String,
        grado_instruccion: GradoInstruccion,
        genero: Genero,
    ) -> Result<Self, PostulanteError> {
        let postulante = Postulante {
            id,
            numero_documento,
            nombre_completo,
            fecha_nacimiento,
            grado_instruccion,
            genero,
            password: Password::new().value(),
        };

        Ok(postulante)
    }
}
