use crate::controller::auth::jwt::Claims;
use crate::controller::error::ApiError;
use crate::controller::postulante::dto::{
    PostulanteDocumentoQuery, PostulanteResponseDTO, build_postulante_links,
};
use crate::controller::postulante::mongo::read::PostulanteReadMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use quizz_auth::autorizacion::domain::value_object::rol::Rol;
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::use_case::buscar_postulante::{InputData, ObtenerPostulantePorId};
use quizz_core::postulante::use_case::buscar_postulante_por_documento::{
    InputData as DocumentoInputData, ObtenerPostulantePorDNI,
};
use quizz_core::postulante::use_case::lista_postulantes::{
    InputData as ListInputData, ObtenerListaDePostulantes, OutputData,
};

pub struct BuscarPostulanteController;

impl BuscarPostulanteController {
    /// `GET /postulantes`: con `id` o con `documento` devuelve un postulante; sin ninguno, la
    /// lista completa (solo personal).
    ///
    /// Un postulante solo puede leer su propio registro: se ignoran `id` y `documento` y se
    /// busca el sujeto del token. Antes la guarda miraba solo `id` y la búsqueda priorizaba
    /// `documento`, así que `?id=<propio>&documento=<ajeno>` devolvía datos de otro (SEC-03).
    pub async fn get(
        req: HttpRequest,
        query: web::Query<PostulanteDocumentoQuery>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let Some(claims) = req.extensions().get::<Claims>().cloned() else {
            return Err(ApiError::NoAutenticado("Token no encontrado".to_string()));
        };
        let repositorio = || Box::new(PostulanteReadMongo::new(db.clone()));

        if claims.rol.as_deref().and_then(|r| r.parse::<Rol>().ok()) == Some(Rol::Postulante) {
            let propio = ObtenerPostulantePorId::new(repositorio())
                .ejecutar(InputData {
                    postulante_id: claims.sub,
                })
                .await?;
            return Ok(HttpResponse::Ok().json(respuesta(propio)));
        }

        let query = query.into_inner();
        let postulante = match (query.id, query.documento) {
            (Some(_), Some(_)) => {
                return Err(ApiError::solicitud_invalida(
                    "Use el parametro id o documento, no ambos",
                ));
            }
            (Some(postulante_id), None) => {
                ObtenerPostulantePorId::new(repositorio())
                    .ejecutar(InputData { postulante_id })
                    .await?
            }
            (None, Some(documento)) => {
                ObtenerPostulantePorDNI::new(repositorio())
                    .ejecutar(DocumentoInputData { documento })
                    .await?
            }
            (None, None) => {
                let lista = ObtenerListaDePostulantes::new(repositorio())
                    .ejecutar(ListInputData {})
                    .await?;
                let items: Vec<PostulanteResponseDTO> =
                    lista.postulantes.into_iter().map(respuesta).collect();
                return Ok(HttpResponse::Ok().json(items));
            }
        };
        Ok(HttpResponse::Ok().json(respuesta(postulante)))
    }
}

fn respuesta(p: OutputData) -> PostulanteResponseDTO {
    PostulanteResponseDTO {
        links: build_postulante_links(&p.id),
        id: p.id,
        documento: p.documento,
        nombre: p.nombre,
        primer_apellido: p.primer_apellido,
        segundo_apellido: p.segundo_apellido,
        nombre_completo: p.nombre_completo,
        fecha_nacimiento: p.fecha_nacimiento,
        grado_instruccion: p.grado_instruccion,
        genero: p.genero,
        fecha_registro: p.fecha_registro,
    }
}
