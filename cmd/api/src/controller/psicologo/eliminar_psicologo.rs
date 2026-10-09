use crate::cache::sesiones::SesionDePsicologo;
use crate::controller::error::ApiError;
use crate::mongo::psicologo::write::PsicologoMongo;
use actix_web::{HttpResponse, web};
use quizz_auth::universal::provider::repositorio::Sesiones;
use quizz_core::psicologo::use_case::eliminar_psicologo::{EliminarPsicologo, InputData};

pub struct EliminarPsicologoController;

impl EliminarPsicologoController {
    /// Borra al psicólogo y cierra su sesión (`204`). Solo el admin tiene permiso
    /// (`psicologo`, `eliminar` en la política). Si Redis falla responde `503` y el psicólogo
    /// queda marcado, sin poder iniciar sesión: repetir la petición completa la eliminación.
    pub async fn delete(
        id: web::Path<String>,
        db: web::Data<mongodb::Database>,
        sesiones: web::Data<dyn Sesiones>,
    ) -> Result<HttpResponse, ApiError> {
        EliminarPsicologo::new(PsicologoMongo::new(db), SesionDePsicologo(sesiones))
            .ejecutar(InputData {
                id: id.into_inner(),
            })
            .await?;
        Ok(HttpResponse::NoContent().finish())
    }
}
