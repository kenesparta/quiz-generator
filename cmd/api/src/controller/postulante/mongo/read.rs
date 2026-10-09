use crate::controller::mongo_repository::MongoRepository;
use crate::controller::postulante::mongo::constantes::POSTULANTE_COLLECTION_NAME;
use actix_web::web;
use chrono::NaiveDateTime;
use futures::TryStreamExt;
use mongodb::bson::{Bson, Document, doc};
use quizz_common::domain::value_objects::fecha_nacimiento::FechaNacimiento;
use quizz_common::domain::value_objects::fecha_registro::FechaRegistro;
use quizz_common::domain::value_objects::zona_horaria::{
    formatear_rfc3339, hora_de_lima, utc_a_lima,
};
use quizz_core::postulante::domain::entity::postulante::Postulante;
use quizz_core::postulante::domain::error::postulante::{PostulanteError, RepositorioError};
use quizz_core::postulante::domain::value_object::documento::Documento;
use quizz_core::postulante::domain::value_object::genero::Genero;
use quizz_core::postulante::domain::value_object::grado_instruccion::GradoInstruccion;
use quizz_core::postulante::domain::value_object::id::PostulanteID;
use quizz_core::postulante::domain::value_object::nombre::Nombre;
use quizz_core::postulante::provider::repositorio::RepositorioPostulanteLectura;
use serde::Deserialize;
use std::str::FromStr;
use tracing::error;

/// Un postulante tal como se guarda. Se lee sin el hash de la contraseña: las lecturas nunca
/// lo necesitan.
#[derive(Deserialize)]
struct FilaPostulante {
    #[serde(rename = "_id")]
    id: String,
    documento: String,
    nombre: String,
    primer_apellido: String,
    segundo_apellido: String,
    fecha_nacimiento: Bson,
    grado_instruccion: String,
    genero: String,
    fecha_registro: Bson,
}

fn sin_password() -> Document {
    doc! { "password": 0 }
}

fn lectura_fallida() -> PostulanteError {
    PostulanteError::PostulanteRepositorioError(RepositorioError::LecturaNoFinalizada)
}

/// Las fechas se guardaron en distintos formatos según la época: RFC 3339, "AAAA-MM-DD
/// HH:MM:SS" en hora de Lima o fecha nativa de BSON. Las tres se devuelven en RFC 3339.
fn leer_fecha(fecha: &Bson) -> Result<String, PostulanteError> {
    match fecha {
        Bson::String(texto) if chrono::DateTime::parse_from_rfc3339(texto).is_ok() => {
            Ok(texto.clone())
        }
        Bson::String(texto) => match NaiveDateTime::parse_from_str(texto, "%Y-%m-%d %H:%M:%S") {
            Ok(local) => hora_de_lima(local)
                .map(|fecha| formatear_rfc3339(&fecha))
                .ok_or_else(lectura_fallida),
            // Una fecha de nacimiento "AAAA-MM-DD" se valida después, en su value object.
            Err(_) => Ok(texto.clone()),
        },
        Bson::DateTime(fecha) => chrono::DateTime::from_timestamp_millis(fecha.timestamp_millis())
            .map(|utc| formatear_rfc3339(&utc_a_lima(utc)))
            .ok_or_else(lectura_fallida),
        _ => Err(lectura_fallida()),
    }
}

impl TryFrom<FilaPostulante> for Postulante {
    type Error = PostulanteError;

    fn try_from(fila: FilaPostulante) -> Result<Self, Self::Error> {
        Ok(Postulante {
            id: PostulanteID::new(&fila.id)?,
            documento: Documento::new(&fila.documento)?,
            nombre_completo: Nombre::new(fila.nombre, fila.primer_apellido, fila.segundo_apellido)?,
            fecha_nacimiento: FechaNacimiento::new(&leer_fecha(&fila.fecha_nacimiento)?)?,
            grado_instruccion: GradoInstruccion::from_str(&fila.grado_instruccion)?,
            genero: Genero::from_str(&fila.genero)?,
            password: None,
            fecha_registro: FechaRegistro::new(&leer_fecha(&fila.fecha_registro)?)?,
        })
    }
}

pub struct PostulanteReadMongo {
    client: web::Data<mongodb::Database>,
}

impl PostulanteReadMongo {
    pub fn new(client: web::Data<mongodb::Database>) -> Self {
        PostulanteReadMongo { client }
    }

    async fn buscar_uno(&self, filtro: Document) -> Result<Postulante, PostulanteError> {
        let fila = self
            .get_collection()
            .clone_with_type::<FilaPostulante>()
            .find_one(filtro)
            .projection(sin_password())
            .await
            .map_err(|e| {
                error!("mongo, leer postulante: {e}");
                lectura_fallida()
            })?
            .ok_or(PostulanteError::PostulanteRepositorioError(
                RepositorioError::RegistroNoEncontrado,
            ))?;
        let id = fila.id.clone();
        Postulante::try_from(fila).inspect_err(|e| error!("postulante {id} ilegible: {e}"))
    }
}

impl MongoRepository for PostulanteReadMongo {
    fn get_collection_name(&self) -> &str {
        POSTULANTE_COLLECTION_NAME
    }

    fn get_db(&self) -> &web::Data<mongodb::Database> {
        &self.client
    }
}

impl RepositorioPostulanteLectura for PostulanteReadMongo {
    async fn obtener_postulante_por_documento(
        &self,
        documento: Documento,
    ) -> Result<Postulante, PostulanteError> {
        self.buscar_uno(doc! { "documento": documento.value() })
            .await
    }

    async fn obtener_postulante_por_id(
        &self,
        postulante_id: PostulanteID,
    ) -> Result<Postulante, PostulanteError> {
        self.buscar_uno(doc! { "_id": postulante_id.to_string() })
            .await
    }

    /// Del más reciente al más antiguo. Un registro ilegible se omite con un error en el log
    /// que nombra su id: no se oculta en silencio, pero tampoco deja sin listado al resto.
    async fn obtener_lista_de_postulantes(&self) -> Result<Vec<Postulante>, PostulanteError> {
        let filas: Vec<FilaPostulante> = self
            .get_collection()
            .clone_with_type::<FilaPostulante>()
            .find(doc! {})
            .projection(sin_password())
            .sort(doc! { "fecha_registro": -1 })
            .await
            .map_err(|e| {
                error!("mongo, listar postulantes: {e}");
                lectura_fallida()
            })?
            .try_collect()
            .await
            .map_err(|e| {
                error!("mongo, leer la lista de postulantes: {e}");
                lectura_fallida()
            })?;

        Ok(filas
            .into_iter()
            .filter_map(|fila| {
                let id = fila.id.clone();
                Postulante::try_from(fila)
                    .inspect_err(|e| error!("postulante {id} ilegible, se omite del listado: {e}"))
                    .ok()
            })
            .collect())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn lee_las_fechas_en_sus_tres_formatos() {
        let rfc = Bson::String("2026-01-05T10:00:00-05:00".to_string());
        assert_eq!(leer_fecha(&rfc).unwrap(), "2026-01-05T10:00:00-05:00");

        let local = Bson::String("2026-01-05 10:00:00".to_string());
        assert!(
            leer_fecha(&local)
                .unwrap()
                .starts_with("2026-01-05T10:00:00")
        );

        let nativa = Bson::DateTime(mongodb::bson::DateTime::from_millis(1_767_625_200_000));
        assert!(
            leer_fecha(&nativa)
                .unwrap()
                .starts_with("2026-01-05T10:00:00")
        );
    }

    #[test]
    fn una_fecha_nativa_anterior_a_1970_no_se_convierte_en_1970() {
        let antigua = Bson::DateTime(mongodb::bson::DateTime::from_millis(-1_500));
        assert!(
            leer_fecha(&antigua)
                .unwrap()
                .starts_with("1969-12-31T18:59:58")
        );
    }
}
