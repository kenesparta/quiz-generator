use crate::postulante::model::error::postulante::PostulanteError;
use crate::postulante::model::value_object::documento::Documento;
use crate::postulante::model::value_object::genero::Genero;
use crate::postulante::model::value_object::grado_instruccion::GradoInstruccion;
use crate::postulante::model::value_object::id::PostulanteID;
use crate::postulante::model::value_object::nombre::Nombre;
use crate::postulante::model::value_object::password::Password;
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;

/// Representa al postulante para obtener la licencia de conducir.
#[derive(Debug)]
pub struct Postulante {
    id: PostulanteID,
    documento: Documento,
    nombre_completo: Nombre,
    fecha_nacimiento: FechaNacimiento,
    grado_instruccion: GradoInstruccion,
    genero: Genero,
    password: Option<Password>,
}

impl Postulante {
    pub fn new(
        id: String,
        documento: String,
        nombre: String,
        apellido_paterno: String,
        apellido_materno: String,
        fecha_nacimiento: String,
        grado_instruccion: GradoInstruccion,
        genero: Genero,
    ) -> Result<Self, PostulanteError> {
        let id = PostulanteID::new(&id)?;
        let documento = Documento::new(documento)?;
        let nombre_completo = Nombre::new(nombre, apellido_paterno, apellido_materno)?;
        let fecha_nacimiento = FechaNacimiento::new(fecha_nacimiento.as_str())?;
        Ok(Postulante {
            id,
            documento,
            nombre_completo,
            fecha_nacimiento,
            grado_instruccion,
            genero,
            password: None,
        })
    }

    pub fn generar_password(self) -> Result<Self, PostulanteError> {
        let password = Some(Password::from_document(&self.documento)?);
        Ok(Postulante {
            id: self.id,
            documento: self.documento,
            nombre_completo: self.nombre_completo,
            fecha_nacimiento: self.fecha_nacimiento,
            grado_instruccion: self.grado_instruccion,
            genero: self.genero,
            password,
        })
    }

    pub fn crear_password_customizado(self, password: String) -> Result<Self, PostulanteError> {
        let password = Some(Password::from_string(password)?);
        Ok(Postulante {
            id: self.id,
            documento: self.documento,
            nombre_completo: self.nombre_completo,
            fecha_nacimiento: self.fecha_nacimiento,
            grado_instruccion: self.grado_instruccion,
            genero: self.genero,
            password,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::postulante::model::error::documento::DocumentoError;
    use crate::postulante::model::error::nombre::NombreError;
    use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimientoError;
    use quizz_common::domain::value_objects::id::IdError;

    #[test]
    fn test_new_postulante_success() {
        let result = Postulante::new(
            "872c8c81-9fab-494a-9267-799876261bcb".to_string(),
            "12345678".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Primaria,
            Genero::Masculino,
        );
        assert!(result.is_ok());
    }

    #[test]
    fn test_new_postulante_invalid_uuid_id() {
        let result = Postulante::new(
            "123e4567-e89b-12d3-a456".to_string(),
            "12345678".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Primaria,
            Genero::Masculino,
        );
        assert!(matches!(
            result.unwrap_err(),
            PostulanteError::PostulanteIdError(IdError::FormatoNoValido(_))
        ));
    }

    #[test]
    fn test_new_postulante_invalid_documento() {
        let result = Postulante::new(
            "e8635835-57fc-458f-8308-2f1bf8c1a0f4".to_string(),
            "".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Secundaria,
            Genero::Masculino,
        );
        assert!(matches!(
            result.unwrap_err(),
            PostulanteError::PostulanteDocumentoError(DocumentoError::DocumentoNoValido)
        ));
    }

    #[test]
    fn test_new_postulante_invalid_nombre() {
        let result = Postulante::new(
            "e3e0081d-19ef-4b7b-8994-84e4aeaa3a49".to_string(),
            "999000999".to_string(),
            "".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Superior,
            Genero::Masculino,
        );
        assert!(matches!(
            result.unwrap_err(),
            PostulanteError::PostulanteNombreError(NombreError::NombreVacio)
        ));
    }

    #[test]
    fn test_new_postulante_invalid_fecha_nacimiento() {
        let result = Postulante::new(
            "500ebdcd-d1c1-4ae8-bb0f-0e7f01547c73".to_string(),
            "45345345".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-12-0".to_string(),
            GradoInstruccion::Ninguno,
            Genero::Masculino,
        );
        assert!(matches!(
            result.unwrap_err(),
            PostulanteError::PostulanteFechaNacimientoError(FechaNacimientoError::FormatoNoValido(
                _
            ))
        ));
    }

    #[test]
    fn test_generar_password_success() {
        let postulante = Postulante::new(
            "872c8c81-9fab-494a-9267-799876261bcb".to_string(),
            "12345678".to_string(),
            "John".to_string(),
            "Doe".to_string(),
            "Smith".to_string(),
            "1990-01-01".to_string(),
            GradoInstruccion::Primaria,
            Genero::Masculino,
        )
        .unwrap();

        let result = postulante.generar_password();
        assert!(result.is_ok());
        assert!(result.unwrap().password.is_some());
    }
}
