use crate::autorizacion::domain::value_object::accion::Accion;
use crate::autorizacion::domain::value_object::recurso::Recurso;
use crate::autorizacion::domain::value_object::rol::Rol;

#[derive(Debug, Clone)]
pub struct SolicitudAcceso {
    pub sujeto: String,
    pub rol: Rol,
    pub recurso: Recurso,
    pub accion: Accion,
}

impl SolicitudAcceso {
    pub fn new(sujeto: String, rol: Rol, recurso: Recurso, accion: Accion) -> Self {
        Self {
            sujeto,
            rol,
            recurso,
            accion,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crear_solicitud_acceso() {
        let solicitud = SolicitudAcceso::new(
            "user-123".to_string(),
            Rol::Psicologo,
            Recurso::Examen,
            Accion::Leer,
        );

        assert_eq!(solicitud.sujeto, "user-123");
        assert_eq!(solicitud.rol, Rol::Psicologo);
        assert_eq!(solicitud.recurso, Recurso::Examen);
        assert_eq!(solicitud.accion, Accion::Leer);
    }
}
