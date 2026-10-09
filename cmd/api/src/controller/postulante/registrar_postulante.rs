use crate::controller::cifrado::Bcrypt;
use crate::controller::error::ApiError;
use crate::controller::postulante::dto::RegistrarPostulanteDTO;
use crate::controller::postulante::mongo::read::PostulanteReadMongo;
use crate::controller::postulante::mongo::write::PostulanteMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use quizz_common::use_case::CasoDeUso;
use quizz_core::postulante::use_case::actualizar_postulante_por_documento::{
    ActualizarPostulantePorDocumento, InputData as ActualizarPorDocumentoInputData,
};
use quizz_core::postulante::use_case::registrar_postulante::{
    InputData, RegistrarPostulantePasswordTemporal,
};

pub struct PostulanteController;

impl PostulanteController {
    pub async fn create(
        id: web::Path<String>,
        body: web::Json<RegistrarPostulanteDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        RegistrarPostulantePasswordTemporal::new(
            Box::new(Bcrypt::default()),
            Box::new(PostulanteMongo::new(db)),
        )
        .ejecutar(InputData {
            id: id.into_inner(),
            documento: dto.documento,
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            fecha_nacimiento: dto.fecha_nacimiento,
            grado_instruccion: dto.grado_instruccion,
            genero: dto.genero,
        })
        .await?;
        Ok(HttpResponse::Created().finish())
    }

    pub async fn update_by_documento(
        body: web::Json<RegistrarPostulanteDTO>,
        db: web::Data<mongodb::Database>,
    ) -> Result<HttpResponse, ApiError> {
        let dto = body.into_inner();
        ActualizarPostulantePorDocumento::new(
            Box::new(PostulanteReadMongo::new(db.clone())),
            Box::new(PostulanteMongo::new(db)),
        )
        .ejecutar(ActualizarPorDocumentoInputData {
            documento: dto.documento,
            nombre: dto.nombre,
            primer_apellido: dto.primer_apellido,
            segundo_apellido: dto.segundo_apellido,
            fecha_nacimiento: dto.fecha_nacimiento,
            grado_instruccion: dto.grado_instruccion,
            genero: dto.genero,
        })
        .await?;
        Ok(HttpResponse::Ok().finish())
    }

    pub async fn remove(
        _req: HttpRequest,
        _body: web::Json<RegistrarPostulanteDTO>,
        _pool: web::Data<mongodb::Database>,
    ) -> HttpResponse {
        HttpResponse::Created().json("")
    }
}
