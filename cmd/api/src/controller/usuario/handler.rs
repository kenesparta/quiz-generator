use crate::controller::usuario::crypto::CifradoUsuario;
use crate::controller::usuario::dto::{RegistrarUsuarioDTO, UsuarioResponseDTO};
use crate::controller::usuario::mongo::write::UsuarioMongo;
use actix_web::{HttpResponse, web};
use quizz_auth::usuario::domain::error::usuario::UsuarioError;
use quizz_auth::usuario::use_case::registrar_usuario::{InputData, RegistrarUsuario};
use quizz_common::use_case::CasoDeUso;
use uuid::Uuid;

pub struct UsuarioController;

impl UsuarioController {
    pub async fn create(
        body: web::Json<RegistrarUsuarioDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let dto = body.into_inner();
        let id = Uuid::new_v4().to_string();

        let use_case =
            RegistrarUsuario::new(Box::new(CifradoUsuario), Box::new(UsuarioMongo::new(pool)));

        let input = InputData {
            id: id.clone(),
            nombre: dto.nombre.clone(),
            email: dto.email.clone(),
            password: dto.password,
            rol: dto.rol.clone(),
        };

        match use_case.ejecutar(input).await {
            Ok(()) => {
                let response = UsuarioResponseDTO {
                    id,
                    nombre: dto.nombre,
                    email: dto.email,
                    rol: dto.rol,
                };
                HttpResponse::Created().json(response)
            }
            Err(err) => {
                match err {
                    UsuarioError::NombreVacio => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": err.to_string()})),
                    UsuarioError::EmailVacio | UsuarioError::EmailInvalido(_) => {
                        HttpResponse::BadRequest()
                            .json(serde_json::json!({"error": err.to_string()}))
                    }
                    UsuarioError::PasswordVacio => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": err.to_string()})),
                    UsuarioError::RolInvalido(_) => HttpResponse::BadRequest()
                        .json(serde_json::json!({"error": err.to_string()})),
                    UsuarioError::ErrorCifrado => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "Error al procesar la contraseña"})),
                    UsuarioError::RepositorioError => HttpResponse::InternalServerError()
                        .json(serde_json::json!({"error": "Error al guardar el usuario"})),
                }
            }
        }
    }
}
