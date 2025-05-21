use crate::controller::postulante::crypto::CifradoPorDefecto;
use crate::controller::postulante::database_write::PostulantePostgres;
use crate::controller::postulante::dto::RegistrarPostulanteDTO;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::use_case::registrar_postulante::{
    InputData, RegistrarPostulantePasswordTemporal,
};
use sqlx::PgPool;

pub struct PostulanteController;

impl PostulanteController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarPostulanteDTO>,
        pool: web::Data<PgPool>,
    ) -> HttpResponse {
        let postulante_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                return HttpResponse::BadRequest().json("no se esta enviando el id del postulante");
            }
        };

        let postulante_pool = PostulantePostgres::new(pool);
        let registrar_postulante = RegistrarPostulantePasswordTemporal::new(
            Box::new(CifradoPorDefecto),
            Box::new(postulante_pool),
        );

        let dto = body.into_inner();
        let input = InputData {
            id: postulante_id,
            documento: dto.documento,
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            fecha_nacimiento: dto.fecha_nacimiento,
            grado_instruccion: dto.grado_instruccion,
            genero: dto.genero,
        };

        match registrar_postulante.ejecutar(input).await {
            Ok(_output) => HttpResponse::Created().json(""),
            Err(err) => match err {
                PostulanteError::PostulanteIdError(id_err) => {
                    HttpResponse::BadRequest().json(format!("Error de ID: {}", id_err))
                }
                PostulanteError::PostulanteDocumentoError(doc_err) => {
                    HttpResponse::BadRequest().json(format!("Error de documento: {}", doc_err))
                }
                PostulanteError::PostulanteNombreError(name_err) => {
                    HttpResponse::BadRequest().json(format!("Error de nombre: {}", name_err))
                }
                PostulanteError::PostulanteGradoInstruccionError(grado_err) => {
                    HttpResponse::BadRequest()
                        .json(format!("Error de grado de instrucción: {}", grado_err))
                }
                PostulanteError::PostulanteGeneroError(genero_err) => {
                    HttpResponse::BadRequest().json(format!("Error de género: {}", genero_err))
                }
                PostulanteError::PostulantePasswordError(_pwd_err) => {
                    // log::error!("Error de password: {}", pwd_err);
                    HttpResponse::InternalServerError().json("Error al procesar la contraseña")
                }
                PostulanteError::PostulanteRepositorioError(_repo_error) => {
                    // log::error!("Error de persistencia {}", repo_error);
                    HttpResponse::InternalServerError().json("Error al guardar el postulante")
                }
                _ => {
                    // log::error!("Error inesperado: {:?}", err);
                    HttpResponse::InternalServerError().json("Error inesperado")
                }
            },
        }
    }

    pub async fn update(
        req: HttpRequest,
        body: web::Json<RegistrarPostulanteDTO>,
        pool: web::Data<PgPool>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }

    pub async fn remove(
        req: HttpRequest,
        body: web::Json<RegistrarPostulanteDTO>,
        pool: web::Data<PgPool>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }
}
