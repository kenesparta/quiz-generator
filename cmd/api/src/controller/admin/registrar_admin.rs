use crate::controller::admin::crypto::CifradoAdmin;
use crate::controller::admin::dto::RegistrarAdminDTO;
use crate::controller::admin::mongo::write::AdminMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::admin::domain::error::admin::AdminError;
use quizz_core::admin::use_case::registrar_admin::{InputData, RegistrarAdmin};

pub struct AdminController;

impl AdminController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarAdminDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let admin_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("POST /admin - id no proporcionado");
                return HttpResponse::BadRequest().json("no se esta enviando el id del admin");
            }
        };

        info!("POST /admin/{}", admin_id);

        let registrar_admin =
            RegistrarAdmin::new(Box::new(CifradoAdmin), Box::new(AdminMongo::new(pool)));

        let dto = body.into_inner();
        let input = InputData {
            id: admin_id.clone(),
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            documento: dto.documento,
            password: dto.password,
        };

        match registrar_admin.ejecutar(input).await {
            Ok(_) => {
                info!("POST /admin/{} - creado exitosamente", admin_id);
                HttpResponse::Created().finish()
            }
            Err(err) => match err {
                AdminError::AdminIdError(ref id_err) => {
                    warn!("POST /admin/{} - error de ID: {}", admin_id, id_err);
                    HttpResponse::BadRequest().json(format!("Error de ID: {}", id_err))
                }
                AdminError::NombreNoValido(ref msg) => {
                    warn!("POST /admin/{} - error de nombre: {}", admin_id, msg);
                    HttpResponse::BadRequest().json(format!("Error de nombre: {}", msg))
                }
                AdminError::DocumentoNoValido(_) => {
                    warn!("POST /admin/{} - error de documento: {}", admin_id, err);
                    HttpResponse::BadRequest().json(format!("Error de documento: {}", err))
                }
                AdminError::PasswordVacio => {
                    error!("POST /admin/{} - error de password", admin_id);
                    HttpResponse::InternalServerError().json("Error al procesar la contraseña")
                }
                AdminError::AdminRepositorioError(ref repo_err) => {
                    error!(
                        "POST /admin/{} - error de repositorio: {:?}",
                        admin_id, repo_err
                    );
                    HttpResponse::InternalServerError().json("Error al guardar el admin")
                }
            },
        }
    }
}
