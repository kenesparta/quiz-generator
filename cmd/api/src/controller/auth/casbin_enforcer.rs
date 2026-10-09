use casbin::{CoreApi, DefaultModel, Enforcer, StringAdapter};
use quizz_auth::autorizacion::domain::entity::solicitud_acceso::SolicitudAcceso;
use quizz_auth::autorizacion::domain::error::autorizacion::AutorizacionError;
use quizz_auth::autorizacion::provider::autorizacion::AutorizacionVerificar;
use tracing::error;

// El modelo y la política van embebidos en el binario: se aplica exactamente lo revisado en
// git y el servidor no depende del directorio desde el que se lance.
const RBAC_MODEL: &str = include_str!("../../../../../rbac/model.conf");
const RBAC_POLICY: &str = include_str!("../../../../../rbac/policy.csv");

/// Construye el enforcer de Casbin con el modelo y la política RBAC del repositorio.
///
/// # Errors
///
/// Falla si el modelo o la política embebidos no son válidos.
pub async fn crear_enforcer() -> casbin::Result<Enforcer> {
    let model = DefaultModel::from_str(RBAC_MODEL).await?;
    Enforcer::new(model, StringAdapter::new(RBAC_POLICY)).await
}

/// Adaptador de [`AutorizacionVerificar`] sobre Casbin. La política es estática, así que el
/// enforcer se comparte sin cerrojo.
pub struct CasbinAutorizacion<'a> {
    enforcer: &'a Enforcer,
}

impl<'a> CasbinAutorizacion<'a> {
    pub fn new(enforcer: &'a Enforcer) -> Self {
        Self { enforcer }
    }
}

impl AutorizacionVerificar for CasbinAutorizacion<'_> {
    fn verificar_permiso(&self, solicitud: &SolicitudAcceso) -> Result<(), AutorizacionError> {
        let permitido = self
            .enforcer
            .enforce(vec![
                solicitud.rol.to_string(),
                solicitud.recurso.to_string(),
                solicitud.accion.to_string(),
            ])
            .map_err(|e| {
                error!("casbin no pudo evaluar la politica: {e}");
                AutorizacionError::ErrorEnforzador
            })?;

        if permitido {
            Ok(())
        } else {
            Err(AutorizacionError::AccesoDenegado)
        }
    }
}

/// La matriz de permisos esperada, escrita a mano: un cambio en `rbac/policy.csv` debe venir
/// con el cambio correspondiente aquí, revisado. La usan también los tests del middleware.
#[cfg(test)]
pub(crate) fn permitido_esperado(
    rol: quizz_auth::autorizacion::domain::value_object::rol::Rol,
    recurso: quizz_auth::autorizacion::domain::value_object::recurso::Recurso,
    accion: quizz_auth::autorizacion::domain::value_object::accion::Accion,
) -> bool {
    use quizz_auth::autorizacion::domain::value_object::accion::Accion::*;
    use quizz_auth::autorizacion::domain::value_object::recurso::Recurso::*;
    use quizz_auth::autorizacion::domain::value_object::rol::Rol;
    match rol {
        Rol::Admin => match recurso {
            Examen | Evaluacion | Postulante | Respuesta | Revision => true,
            Psicologo => matches!(accion, Leer | Escribir | Eliminar),
            Admin => matches!(accion, Leer | Escribir),
        },
        Rol::Psicologo => match recurso {
            Examen | Evaluacion | Postulante | Revision => {
                matches!(accion, Leer | Escribir | Actualizar)
            }
            Respuesta => accion == Leer,
            Psicologo | Admin => false,
        },
        Rol::Postulante => match recurso {
            Postulante => accion == Leer,
            Respuesta => matches!(accion, Leer | Escribir | Actualizar),
            _ => false,
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use quizz_auth::autorizacion::domain::value_object::accion::Accion;
    use quizz_auth::autorizacion::domain::value_object::recurso::Recurso;
    use quizz_auth::autorizacion::domain::value_object::rol::Rol;

    const RECURSOS: [Recurso; 7] = [
        Recurso::Admin,
        Recurso::Examen,
        Recurso::Evaluacion,
        Recurso::Postulante,
        Recurso::Psicologo,
        Recurso::Respuesta,
        Recurso::Revision,
    ];
    const ACCIONES: [Accion; 4] = [
        Accion::Leer,
        Accion::Escribir,
        Accion::Actualizar,
        Accion::Eliminar,
    ];

    #[tokio::test]
    async fn la_politica_coincide_con_la_matriz_esperada() {
        let enforcer = crear_enforcer().await.unwrap();
        let autorizacion = CasbinAutorizacion::new(&enforcer);

        for rol in [Rol::Admin, Rol::Psicologo, Rol::Postulante] {
            for recurso in RECURSOS {
                for accion in ACCIONES {
                    let solicitud = SolicitudAcceso::new("sub".to_string(), rol, recurso, accion);
                    let resultado = autorizacion.verificar_permiso(&solicitud);
                    let esperado = permitido_esperado(rol, recurso, accion);
                    assert_eq!(
                        resultado.is_ok(),
                        esperado,
                        "rol={rol} recurso={recurso} accion={accion}"
                    );
                    if !esperado {
                        assert_eq!(resultado, Err(AutorizacionError::AccesoDenegado));
                    }
                }
            }
        }
    }
}
