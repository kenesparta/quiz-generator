use crate::postulante::domain::error::documento::DocumentoError;
use crate::postulante::domain::value_object::documento::Documento;

pub fn obtener_password_del_documento(documento: Documento) -> Result<String, DocumentoError> {
    let tmp_pass = documento.obtener_ultimos_cuatro_caracteres()?;
    Ok(tmp_pass)
}
