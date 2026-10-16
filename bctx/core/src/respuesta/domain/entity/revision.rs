use crate::admin::domain::value_object::id::AdminID;
use crate::psicologo::domain::value_object::id::PsicologoID;
use crate::respuesta::domain::value_object::id::RespuestaID;
use chrono::{DateTime, FixedOffset};

pub struct ExamenRevision {
    pub examen_id: String,
    pub observacion: String,
}

/// Quién califica una hoja: el sujeto del token.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Revisor {
    /// Firma la revisión con su nombre y su colegiatura.
    Psicologo(PsicologoID),
    /// Califica sin firma: no tiene colegiatura.
    Admin(AdminID),
}

/// La firma de una revisión: el nombre y la colegiatura del psicólogo tal como estaban al
/// calificar. Se guarda en la hoja, así que no cambia ni se pierde si después se elimina al
/// psicólogo.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Firma {
    pub nombre_completo: String,
    pub colegiatura: String,
}

impl Firma {
    /// Une nombre y apellidos con un espacio, sin dejar huecos si falta el segundo apellido.
    pub fn new(
        nombre: &str,
        primer_apellido: &str,
        segundo_apellido: &str,
        colegiatura: &str,
    ) -> Self {
        let nombre_completo = [nombre, primer_apellido, segundo_apellido]
            .map(str::trim)
            .into_iter()
            .filter(|parte| !parte.is_empty())
            .collect::<Vec<_>>()
            .join(" ");
        Self {
            nombre_completo,
            colegiatura: colegiatura.trim().to_string(),
        }
    }
}

/// La calificación de una hoja finalizada, con quién la hizo y cuándo.
pub struct RevisionRealizada {
    pub respuesta_id: RespuestaID,
    pub evaluacion_id: String,
    pub examenes: Vec<ExamenRevision>,
    pub resultado: String,
    /// Id del psicólogo o admin que califica (el sujeto del token).
    pub revisado_por: String,
    /// La firma del psicólogo que califica; `None` si califica un admin.
    pub firma: Option<Firma>,
    pub fecha: DateTime<FixedOffset>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn la_firma_une_nombre_y_apellidos() {
        let firma = Firma::new("Maria", "Garcia", "Lopez", "CPsP-123");
        assert_eq!(firma.nombre_completo, "Maria Garcia Lopez");
        assert_eq!(firma.colegiatura, "CPsP-123");
    }

    #[test]
    fn sin_segundo_apellido_no_quedan_espacios_de_mas() {
        let firma = Firma::new(" Maria ", "Garcia", "  ", " CPsP-123 ");
        assert_eq!(firma.nombre_completo, "Maria Garcia");
        assert_eq!(firma.colegiatura, "CPsP-123");
    }
}
