use crate::postulante::model::errors::PostulanteError;
use uuid::Uuid;

/// Representa el ID unico del postulante
#[derive(Debug, PartialEq)]
pub struct PostulanteID(String);

impl PostulanteID {
    pub fn new(id: String) -> Result<Self, PostulanteError> {
        let postulante_id = PostulanteID(id);
        postulante_id.asegurar_postulante_id_es_valido()?;
        Ok(postulante_id)
    }

    fn asegurar_postulante_id_es_valido(&self) -> Result<(), PostulanteError> {
        if self.0.trim().is_empty() {
            return Err(PostulanteError::IdEsVacio);
        }

        if Uuid::parse_str(&self.0).is_err() {
            return Err(PostulanteError::IdNoValido);
        }

        Ok(())
    }

    pub fn value(&self) -> &String {
        &self.0
    }
}

/// Representa al postulante para obtener la licencia de conducir.
/// Este postulante es creado para poder realizar el examen
pub struct Postulante {
    id: PostulanteID,
    numero_documento: String,
    nombre_completo: String,

    /// Este password es temporal y se genera en el momento de crear el postulante
    password: String,
}
