use crate::controller::psicologo::crypto::CifradoPsicologo;
use crate::controller::psicologo::dto::RegistrarPsicologoDTO;
use crate::controller::psicologo::mongo::write::PsicologoMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info, warn};
use quizz_common::use_case::CasoDeUso;
use quizz_core::psicologo::domain::error::psicologo::PsicologoError;
use quizz_core::psicologo::use_case::registrar_psicologo::{InputData, RegistrarPsicologo};

pub struct PsicologoController;

impl PsicologoController {
    pub async fn create(
        req: HttpRequest,
        body: web::Json<RegistrarPsicologoDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let psicologo_id = match req.match_info().get("id") {
            Some(id) => id.to_string(),
            None => {
                warn!("POST /psicologo - id no proporcionado");
                return HttpResponse::BadRequest().json("no se esta enviando el id del psicologo");
            }
        };

        info!("POST /psicologo/{}", psicologo_id);

        let registrar_psicologo = RegistrarPsicologo::new(
            Box::new(CifradoPsicologo),
            Box::new(PsicologoMongo::new(pool)),
        );

        let dto = body.into_inner();
        let input = InputData {
            id: psicologo_id.clone(),
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            email: dto.email,
            especialidad: dto.especialidad,
            password: dto.password,
        };

        match registrar_psicologo.ejecutar(input).await {
            Ok(_) => {
                info!("POST /psicologo/{} - creado exitosamente", psicologo_id);
                HttpResponse::Created().finish()
            }
            Err(err) => match err {
                PsicologoError::PsicologoIdError(ref id_err) => {
                    warn!("POST /psicologo/{} - error de ID: {}", psicologo_id, id_err);
                    HttpResponse::BadRequest().json(format!("Error de ID: {}", id_err))
                }
                PsicologoError::NombreNoValido(ref msg) => {
                    warn!(
                        "POST /psicologo/{} - error de nombre: {}",
                        psicologo_id, msg
                    );
                    HttpResponse::BadRequest().json(format!("Error de nombre: {}", msg))
                }
                PsicologoError::EmailVacio | PsicologoError::EmailNoValido(_) => {
                    warn!("POST /psicologo/{} - error de email: {}", psicologo_id, err);
                    HttpResponse::BadRequest().json(format!("Error de email: {}", err))
                }
                PsicologoError::EspecialidadVacia => {
                    warn!("POST /psicologo/{} - especialidad vacia", psicologo_id);
                    HttpResponse::BadRequest().json("La especialidad es requerida")
                }
                PsicologoError::PasswordVacio => {
                    error!("POST /psicologo/{} - error de password", psicologo_id);
                    HttpResponse::InternalServerError().json("Error al procesar la contraseña")
                }
                PsicologoError::PsicologoRepositorioError(ref repo_err) => {
                    error!(
                        "POST /psicologo/{} - error de repositorio: {:?}",
                        psicologo_id, repo_err
                    );
                    HttpResponse::InternalServerError().json("Error al guardar el psicologo")
                }
            },
        }
    }
}
