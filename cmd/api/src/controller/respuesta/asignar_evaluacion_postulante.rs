use crate::controller::respuesta::dto::AsociarRespuestaDTO;
use crate::controller::respuesta::mongo::write::RespuestaEvaluacionMongo;
use actix_web::{HttpRequest, HttpResponse, web};
use log::{error, info};
use quizz_common::use_case::CasoDeUso;
use quizz_core::respuesta::use_case::asignar_postulante::{
    AsignarEvaluacionAPostulante, InputData,
};

pub struct AsignarEvaluacionPostulanteController;

impl AsignarEvaluacionPostulanteController {
    pub async fn create(
        _req: HttpRequest,
        body: web::Json<AsociarRespuestaDTO>,
        pool: web::Data<mongodb::Client>,
    ) -> HttpResponse {
        let asociar =
            AsignarEvaluacionAPostulante::new(Box::new(RespuestaEvaluacionMongo::new(pool)));
        let dto = body.into_inner();

        info!(
            "POST /respuesta - asignar evaluacion={} a postulante={}",
            dto.evaluacion_id, dto.postulante_id
        );

        let input = InputData {
            evaluacion_id: dto.evaluacion_id,
            postulante_id: dto.postulante_id,
        };

        match asociar.ejecutar(input).await {
            Ok(()) => {
                info!("POST /respuesta - asignacion exitosa");
                HttpResponse::Ok().finish()
            }
            Err(err) => {
                error!("POST /respuesta - error al asignar: {}", err);
                HttpResponse::InternalServerError().body(err.to_string())
            }
        }
    }
}
