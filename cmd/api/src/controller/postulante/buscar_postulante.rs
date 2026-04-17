use crate::controller::auth::jwt::Claims;
use crate::controller::postulante::dto::{
    PostulanteDocumentoQuery, PostulanteResponseDTO, build_postulante_links,
};
use crate::controller::postulante::mongo::read::PostulanteReadMongo;
use actix_web::{HttpMessage, HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::use_case::buscar_postulante::{
    InputData, ObtenerPostulantePorDocumento,
};
use quizz_core::postulante::use_case::buscar_postulante_por_documento::{
    InputData as DocumentoInputData, ObtenerPostulantePorDNI,
};
use quizz_core::postulante::use_case::lista_postulantes::{
    InputData as ListInputData, ObtenerListaDePostulantes,
};

pub struct PostulanteObtenerPorDocumentoController;
impl PostulanteObtenerPorDocumentoController {
    pub async fn get(
        req: HttpRequest,
        query: web::Query<PostulanteDocumentoQuery>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        // Si el usuario es postulante, solo puede consultar su propia informacion
        if let Some(claims) = req.extensions().get::<Claims>()
            && let Some(ref rol) = claims.rol
            && rol == "postulante"
        {
            match &query.id {
                Some(id) if *id != claims.sub => {
                    warn!(
                        "GET /postulantes - postulante {} intento acceder a datos de {}",
                        claims.sub, id
                    );
                    return HttpResponse::Forbidden().json(
                        serde_json::json!({"error": "Solo puede consultar su propia informacion"}),
                    );
                }
                None => {
                    warn!(
                        "GET /postulantes - postulante {} intento listar todos los postulantes",
                        claims.sub
                    );
                    return HttpResponse::Forbidden().json(
                        serde_json::json!({"error": "No tiene permiso para listar todos los postulantes"}),
                    );
                }
                _ => {}
            }
        }

        if let Some(ref documento) = query.documento {
            return PostulanteBuscarPorDocumentoController::get(documento.clone(), pool).await;
        }

        let postulante_id = match &query.id {
            Some(id) => id.clone(),
            None => return PostulanteListController::get(pool).await,
        };

        info!("GET /postulantes?id={}", postulante_id);

        let obtener_postulante =
            ObtenerPostulantePorDocumento::new(Box::new(PostulanteReadMongo::new(pool)));

        match obtener_postulante
            .ejecutar(InputData {
                postulante_id: postulante_id.clone(),
            })
            .await
        {
            Ok(output) => {
                info!("GET /postulantes?id={} - encontrado", postulante_id);
                let links = build_postulante_links(&output.id, &output.documento);
                HttpResponse::Ok().json(PostulanteResponseDTO {
                    id: output.id.to_string(),
                    documento: output.documento.to_string(),
                    nombre: output.nombre.to_string(),
                    primer_apellido: output.primer_apellido.to_string(),
                    segundo_apellido: output.segundo_apellido.to_string(),
                    nombre_completo: output.nombre_completo.to_string(),
                    fecha_nacimiento: output.fecha_nacimiento.to_string(),
                    grado_instruccion: output.grado_instruccion.to_string(),
                    genero: output.genero.to_string(),
                    fecha_registro: output.fecha_registro.to_string(),
                    links,
                })
            }
            Err(err) => {
                warn!("GET /postulantes?id={} - error: {:?}", postulante_id, err);
                match err {
                    PostulanteError::PostulanteDocumentoError(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": "Invalid document format"})),
                    PostulanteError::PostulanteRepositorioError(_) => HttpResponse::NotFound()
                        .json(serde_json::json!({"error": "Postulante not found"})),
                    PostulanteError::PostulanteIdError(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": "Invalid ID format"})),
                    PostulanteError::PostulanteNombreError(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": "Invalid name format"})),
                    PostulanteError::PostulanteFechaNacimientoError(_) => {
                        HttpResponse::BadRequest()
                            .json(serde_json::json!({"error": "Invalid birth date"}))
                    }
                    PostulanteError::PostulantePasswordError(_) => HttpResponse::Unauthorized()
                        .json(serde_json::json!({"error": "Password error"})),
                    PostulanteError::PostulanteGradoInstruccionError(_) => {
                        HttpResponse::BadRequest()
                            .json(serde_json::json!({"error": "Invalid education level"}))
                    }
                    PostulanteError::PostulanteGeneroError(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": "Invalid gender"})),
                    _ => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "An unexpected error occurred"})),
                }
            }
        }
    }
}

pub struct PostulanteBuscarPorDocumentoController;
impl PostulanteBuscarPorDocumentoController {
    pub async fn get(documento: String, pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /postulantes?documento={}", documento);

        let obtener_postulante =
            ObtenerPostulantePorDNI::new(Box::new(PostulanteReadMongo::new(pool)));

        match obtener_postulante
            .ejecutar(DocumentoInputData {
                documento: documento.clone(),
            })
            .await
        {
            Ok(output) => {
                info!("GET /postulantes?documento={} - encontrado", documento);
                let links = build_postulante_links(&output.id, &output.documento);
                HttpResponse::Ok().json(PostulanteResponseDTO {
                    id: output.id.to_string(),
                    documento: output.documento.to_string(),
                    nombre: output.nombre.to_string(),
                    primer_apellido: output.primer_apellido.to_string(),
                    segundo_apellido: output.segundo_apellido.to_string(),
                    nombre_completo: output.nombre_completo.to_string(),
                    fecha_nacimiento: output.fecha_nacimiento.to_string(),
                    grado_instruccion: output.grado_instruccion.to_string(),
                    genero: output.genero.to_string(),
                    fecha_registro: output.fecha_registro.to_string(),
                    links,
                })
            }
            Err(err) => {
                warn!(
                    "GET /postulantes?documento={} - error: {:?}",
                    documento, err
                );
                match err {
                    PostulanteError::PostulanteDocumentoError(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": "Invalid document format"})),
                    PostulanteError::PostulanteRepositorioError(_) => HttpResponse::NotFound()
                        .json(serde_json::json!({"error": "Postulante not found"})),
                    _ => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "An unexpected error occurred"})),
                }
            }
        }
    }
}

pub struct PostulanteListController;
impl PostulanteListController {
    pub async fn get(pool: web::Data<mongodb::Client>) -> HttpResponse {
        info!("GET /postulantes - listar todos");

        let postulante_pool = PostulanteReadMongo::new(pool);
        let lista_de_postulantes = ObtenerListaDePostulantes::new(Box::new(postulante_pool));
        match lista_de_postulantes.ejecutar(ListInputData {}).await {
            Ok(postulantes) => {
                info!(
                    "GET /postulantes - {} resultados",
                    postulantes.postulantes.len()
                );
                let response_dto: Vec<PostulanteResponseDTO> = postulantes
                    .postulantes
                    .into_iter()
                    .map(|p| {
                        let id = p.id.to_string();
                        let documento = p.documento.to_string();
                        let links = build_postulante_links(&id, &documento);
                        PostulanteResponseDTO {
                            id,
                            documento: p.documento.to_string(),
                            nombre: p.nombre.to_string(),
                            primer_apellido: p.primer_apellido.to_string(),
                            segundo_apellido: p.segundo_apellido.to_string(),
                            nombre_completo: p.nombre_completo.to_string(),
                            fecha_nacimiento: p.fecha_nacimiento.to_string(),
                            grado_instruccion: p.grado_instruccion.to_string(),
                            genero: p.genero.to_string(),
                            fecha_registro: p.fecha_registro.to_string(),
                            links,
                        }
                    })
                    .collect();

                HttpResponse::Ok().json(response_dto)
            }
            Err(err) => {
                error!("GET /postulantes - error al listar: {:?}", err);
                match err {
                    PostulanteError::PostulanteRepositorioError(_) => {
                        HttpResponse::InternalServerError()
                            .json(serde_json::json!({"error": "Error fetching postulantes"}))
                    }
                    _ => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "An unexpected error occurred"})),
                }
            }
        }
    }
}
