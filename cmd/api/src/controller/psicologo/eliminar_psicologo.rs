use crate::controller::error::ApiError;
use crate::mongo::psicologo::write::PsicologoMongo;
use actix_web::{HttpResponse, web};
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_core::psicologo::use_case::eliminar_psicologo::{EliminarPsicologo, InputData};
use tracing::error;

pub struct EliminarPsicologoController;

impl EliminarPsicologoController {
    /// Borra al psicólogo y cierra su sesión: su token deja de autenticar en ese momento, no
    /// cuando expire. Solo el admin tiene permiso (`psicologo`, `eliminar` en la política).
    pub async fn delete(
        id: web::Path<String>,
        db: web::Data<mongodb::Database>,
        sesiones: web::Data<dyn Sesiones>,
    ) -> Result<HttpResponse, ApiError> {
        let id = EliminarPsicologo::new(PsicologoMongo::new(db))
            .ejecutar(InputData {
                id: id.into_inner(),
            })
            .await?;

        // La sesión se cierra después del borrado: así solo se revoca la de un psicólogo que
        // existía. Si Redis falla justo aquí, su token vale hasta que expire.
        sesiones.revocar(&id.to_string()).await.map_err(|e| {
            error!("psicologo {id} eliminado, pero no se pudo cerrar su sesion: {e}");
            ApiError::NoDisponible
        })?;

        Ok(HttpResponse::NoContent().finish())
    }
}
