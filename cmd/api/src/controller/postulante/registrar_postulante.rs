use crate::controller::postulante::crypto::CifradoPorDefecto;
use crate::controller::postulante::dto::RegistrarPostulanteDTO;
use crate::controller::postulante::mongo::write::PostulanteMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::domain::error::postulante::PostulanteError;
use quizz_core::postulante::use_case::registrar_postulante::{
    InputData, RegistrarPostulantePasswordTemporal,
};

pub struct PostulanteController;

impl PostulanteController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarPostulanteDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let postulante_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("POST /postulante - id no proporcionado");
                return HttpResponse::BadRequest().json("no se esta enviando el id del postulante");
            }
        };

        info!("POST /postulante/{}", postulante_id);

        let registrar_postulante = RegistrarPostulantePasswordTemporal::new(
            Box::new(CifradoPorDefecto),
            Box::new(PostulanteMongo::new(pool)),
        );

        let dto = body.into_inner();
        let input = InputData {
            id: postulante_id.clone(),
            documento: dto.documento,
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            fecha_nacimiento: dto.fecha_nacimiento,
            grado_instruccion: dto.grado_instruccion,
            genero: dto.genero,
        };

        match registrar_postulante.ejecutar(input).await {
            Ok(_output) => {
                info!("POST /postulante/{} - creado exitosamente", postulante_id);
                HttpResponse::Created().finish()
            }
            Err(err) => match err {
                PostulanteError::PostulanteIdError(ref id_err) => {
                    warn!(
                        "POST /postulante/{} - error de ID: {}",
                        postulante_id, id_err
                    );
                    HttpResponse::BadRequest().json(format!("Error de ID: {}", id_err))
                }
                PostulanteError::PostulanteDocumentoError(ref doc_err) => {
                    warn!(
                        "POST /postulante/{} - error de documento: {}",
                        postulante_id, doc_err
                    );
                    HttpResponse::BadRequest().json(format!("Error de documento: {}", doc_err))
                }
                PostulanteError::PostulanteNombreError(ref name_err) => {
                    warn!(
                        "POST /postulante/{} - error de nombre: {}",
                        postulante_id, name_err
                    );
                    HttpResponse::BadRequest().json(format!("Error de nombre: {}", name_err))
                }
                PostulanteError::PostulanteGradoInstruccionError(ref grado_err) => {
                    warn!(
                        "POST /postulante/{} - error de grado de instruccion: {}",
                        postulante_id, grado_err
                    );
                    HttpResponse::BadRequest()
                        .json(format!("Error de grado de instrucción: {}", grado_err))
                }
                PostulanteError::PostulanteGeneroError(ref genero_err) => {
                    warn!(
                        "POST /postulante/{} - error de genero: {}",
                        postulante_id, genero_err
                    );
                    HttpResponse::BadRequest().json(format!("Error de género: {}", genero_err))
                }
                PostulanteError::PostulantePasswordError(ref pwd_err) => {
                    error!(
                        "POST /postulante/{} - error de password: {}",
                        postulante_id, pwd_err
                    );
                    HttpResponse::InternalServerError().json("Error al procesar la contraseña")
                }
                PostulanteError::PostulanteRepositorioError(ref repo_err) => {
                    error!(
                        "POST /postulante/{} - error de repositorio: {:?}",
                        postulante_id, repo_err
                    );
                    HttpResponse::InternalServerError().json("Error al guardar el postulante")
                }
                _ => {
                    error!(
                        "POST /postulante/{} - error inesperado: {:?}",
                        postulante_id, err
                    );
                    HttpResponse::InternalServerError().json("Error inesperado")
                }
            },
        }
    }

    pub async fn update(
        _req: HttpRequest,
        _body: web::Json<RegistrarPostulanteDTO>,
        _pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }

    pub async fn remove(
        _req: HttpRequest,
        _body: web::Json<RegistrarPostulanteDTO>,
        _pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }
}
