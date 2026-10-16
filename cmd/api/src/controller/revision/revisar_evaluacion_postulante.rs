use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links};
use crate::controller::revision::dto::{CrearRevisionDTO, RevisionCreatedDTO};
use crate::mongo::psicologo::write::PsicologoMongo;
use crate::mongo::respuesta::RespuestaMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::domain::value_objects::zona_horaria::ahora_lima;
use quizz_core::admin::domain::value_object::id::AdminID;
use quizz_core::psicologo::domain::value_object::id::PsicologoID;
use quizz_core::respuesta::domain::entity::respuesta::Revision;
use quizz_core::respuesta::domain::entity::revision::Revisor;
use quizz_core::respuesta::use_case::realizar_revision::{
    InputData, InputDataExamen, RealizarRevision,
};

pub struct RevisarEvaluacionPostulanteController;

impl RevisarEvaluacionPostulanteController {
    /// Califica una hoja finalizada (409 si todavía está en curso) y registra quién y cuándo.
    /// Si califica un psicólogo, la revisión queda firmada con su nombre y su colegiatura, y
    /// la respuesta la incluye.
    pub async fn review(
        req: HttpRequest,
        respuesta_id: web::Path<String>,
        body: web::Json<CrearRevisionDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let revisor = match claims.rol.as_deref().and_then(|r| r.parse::<Rol>().ok()) {
            Some(Rol::Psicologo) => Revisor::Psicologo(PsicologoID::new(&claims.sub)?),
            Some(Rol::Admin) => Revisor::Admin(AdminID::new(&claims.sub)?),
            Some(Rol::Postulante) | None => return Err(ApiError::prohibido("Rol no valido")),
        };
        let respuesta_id = respuesta_id.into_inner();
        let body = body.into_inner();

        let revision =
            RealizarRevision::new(RespuestaMongo::new(db.clone()), PsicologoMongo::new(db))
                .ejecutar(InputData {
                    respuesta_id: respuesta_id.clone(),
                    evaluacion_id: body.evaluacion_id,
                    resultado: body.resultado,
                    examenes: body
                        .examenes
                        .into_iter()
                        .map(|ex| InputDataExamen {
                            examen_id: ex.examen_id,
                            observacion: ex.observacion,
                        })
                        .collect(),
                    revisor,
                    ahora: ahora_lima(),
                })
                .await?;

        let mut links = Links::new();
        links.insert(
            "self".into(),
            Link::get(format!("/revisiones/{respuesta_id}")),
        );
        links.insert(
            "respuesta".into(),
            Link::get(format!("/respuestas/{respuesta_id}")),
        );

        Ok(HttpResponse::Created().json(RevisionCreatedDTO {
            respuesta_id,
            estado_revision: Revision::Finalizada.to_string(),
            psicologo: revision.firma.map(Into::into),
            links,
        }))
    }
}
