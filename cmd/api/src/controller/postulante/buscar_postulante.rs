use crate::controller::postulante::database_read::PostulanteReadPostgres;
use crate::controller::postulante::dto::{Links, PostulanteDocumentoQuery, PostulanteResponseDTO};
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::use_case::buscar_postulante::{
    InputData, ObtenerPostulantePorDocumento,
};
use quizz_core::postulante::use_case::lista_postulantes::{InputData as ListInputData, ObtenerListaDePostulantes};
use sqlx::PgPool;

pub struct PostulanteObtenerPorDocumentoController;
impl PostulanteObtenerPorDocumentoController {
    pub async fn get(
        query: web::Query<PostulanteDocumentoQuery>,
        pool: web::Data<PgPool>,
    ) -> HttpResponse {
        let postulante_documento = &query.documento;
        let postulante_pool = PostulanteReadPostgres::new(pool);
        let obtener_postulante = ObtenerPostulantePorDocumento::new(Box::new(postulante_pool));

        match obtener_postulante
            .ejecutar(InputData {
                documento: postulante_documento.to_string(),
            })
            .await
        {
            Ok(output) => HttpResponse::Ok().json(PostulanteResponseDTO {
                id: output.id.to_string(),
                documento: output.documento.to_string(),
                nombre: output.nombre.to_string(),
                primer_apellido: output.primer_apellido.to_string(),
                segundo_apellido: output.segundo_apellido.to_string(),
                nombre_completo: output.nombre_completo.to_string(),
                fecha_nacimiento: output.fecha_nacimiento.to_string(),
                grado_instruccion: output.grado_instruccion.to_string(),
                genero: output.genero.to_string(),
                links_: Links {
                    self_: "".to_string(),
                    update: "".to_string(),
                    delete: "".to_string(),
                    exams: "".to_string(),
                    results: "".to_string(),
                },
            }),
            Err(err) => match err {
                PostulanteError::PostulanteDocumentoError(_) => HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Invalid document format"})),
                PostulanteError::PostulanteRepositorioError(_) => HttpResponse::NotFound()
                    .json(serde_json::json!({"error": "Postulante not found"})),
                PostulanteError::PostulanteIdError(_) => HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Invalid ID format"})),
                PostulanteError::PostulanteNombreError(_) => HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Invalid name format"})),
                PostulanteError::PostulanteFechaNacimientoError(_) => HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Invalid birth date"})),
                PostulanteError::PostulantePasswordError(_) => HttpResponse::Unauthorized()
                    .json(serde_json::json!({"error": "Password error"})),
                PostulanteError::PostulanteGradoInstruccionError(_) => HttpResponse::BadRequest()
                    .json(serde_json::json!({"error": "Invalid education level"})),
                PostulanteError::PostulanteGeneroError(_) => {
                    HttpResponse::BadRequest().json(serde_json::json!({"error": "Invalid gender"}))
                }
            },
        }
    }
}

pub struct PostulanteListController;
impl PostulanteListController {
    pub async fn get(_req: HttpRequest, pool: web::Data<PgPool>) -> HttpResponse {
        let postulante_pool = PostulanteReadPostgres::new(pool);
        let lista_de_postulantes = ObtenerListaDePostulantes::new(Box::new(postulante_pool));
        match lista_de_postulantes.ejecutar(ListInputData {}).await {
            Ok(postulantes) => {
                let response_dto: Vec<PostulanteResponseDTO> = postulantes.postulantes
                    .into_iter()
                    .map(|p| PostulanteResponseDTO {
                        id: p.id.to_string(),
                        documento: p.documento.to_string(),
                        nombre: p.nombre.to_string(),
                        primer_apellido: p.primer_apellido.to_string(),
                        segundo_apellido: p.segundo_apellido.to_string(),
                        nombre_completo: p.nombre_completo.to_string(),
                        fecha_nacimiento: p.fecha_nacimiento.to_string(),
                        grado_instruccion: p.grado_instruccion.to_string(),
                        genero: p.genero.to_string(),
                        links_: Links {
                            self_: format!("/postulantes/{}", p.id),
                            update: format!("/postulantes/{}", p.id),
                            delete: format!("/postulantes/{}", p.id),
                            exams: format!("/postulantes/{}/exams", p.id),
                            results: format!("/postulantes/{}/results", p.id),
                        },
                    })
                    .collect();

                HttpResponse::Ok().json(response_dto)
            },
            Err(err) => match err {
                PostulanteError::PostulanteRepositorioError(_) => HttpResponse::InternalServerError()
                    .json(serde_json::json!({"error": "Error fetching postulantes"})),
                _ => HttpResponse::InternalServerError()
                    .json(serde_json::json!({"error": "An unexpected error occurred"})),
            }
        }
    }
}
