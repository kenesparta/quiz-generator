use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::hateoas::{Link, Links, ListResponse, enlaces};
use crate::controller::respuesta::dto::{AsignacionListItemDTO, AsignacionesQueryParams};
use crate::controller::respuesta::mongo::read::ListarAsignacionesMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::listar_asignaciones::{InputData, ListarAsignaciones};

pub struct ListarAsignacionesController;

impl ListarAsignacionesController {
    pub async fn list(
        req: HttpRequest,
        query: web::Query<AsignacionesQueryParams>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        // El scope ya exige leer respuestas; este listado, además, es solo para el personal.
        let rol = req
            .extensions()
            .get::<Claims>()
            .and_then(|c| c.rol.as_deref()?.parse::<Rol>().ok());
        if !matches!(rol, Some(Rol::Admin | Rol::Psicologo)) {
            return Err(ApiError::prohibido("Acceso denegado"));
        }

        let query = query.into_inner();
        let asignaciones = ListarAsignaciones::new(Box::new(ListarAsignacionesMongo::new(db)))
            .ejecutar(InputData {
                postulante_id: query.postulante_id,
                evaluacion_id: query.evaluacion_id,
            })
            .await?;

        let items: Vec<AsignacionListItemDTO> = asignaciones
            .into_iter()
            .map(|a| {
                let links = enlaces::asignacion(&a.respuesta_id, &a.postulante_id);

                let nombre_completo = [
                    a.postulante_nombre.as_str(),
                    a.postulante_primer_apellido.as_str(),
                    a.postulante_segundo_apellido.as_str(),
                ]
                .iter()
                .filter(|s| !s.is_empty())
                .copied()
                .collect::<Vec<&str>>()
                .join(" ");

                AsignacionListItemDTO {
                    id: a.respuesta_id,
                    estado: a.estado,
                    fecha_tiempo_inicio: a.fecha_tiempo_inicio,
                    fecha_tiempo_fin: a.fecha_tiempo_fin,
                    evaluacion_id: a.evaluacion_id,
                    evaluacion_nombre: a.evaluacion_nombre,
                    evaluacion_descripcion: a.evaluacion_descripcion,
                    postulante_id: a.postulante_id,
                    postulante_documento: a.postulante_documento,
                    postulante_nombre: a.postulante_nombre,
                    postulante_primer_apellido: a.postulante_primer_apellido,
                    postulante_segundo_apellido: a.postulante_segundo_apellido,
                    postulante_nombre_completo: nombre_completo,
                    links,
                }
            })
            .collect();

        let mut collection_links = Links::new();
        collection_links.insert("self".into(), Link::get("/respuestas/asignaciones"));

        Ok(HttpResponse::Ok().json(ListResponse {
            links: collection_links,
            items,
        }))
    }
}
