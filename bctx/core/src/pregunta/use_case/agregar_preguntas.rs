use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::entity::pregunta::PreguntaEntity;
use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::service::lista_preguntas::ListaDePreguntas;
use crate::pregunta::provider::repositorio::RepositorioAgregarPregunta;
use async_trait::async_trait;
use quizz_common::use_case::CasoDeUso;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct InputData {
    pub examen_id: String,
    pub preguntas: Vec<PreguntaEntityInput>,
}

#[derive(Debug, Clone)]
pub struct PreguntaEntityInput {
    pub id: String,
    pub contenido: String,
    pub etiqueta: String,
    pub tipo_de_pregunta: String,
    pub imagen_ref: Option<String>,
    pub alternativas: HashMap<String, String>,
    pub puntos: HashMap<String, u32>,
}

pub struct AgregarPreguntas<RepoErr> {
    reposotorio: Box<dyn RepositorioAgregarPregunta<RepoErr>>,
}

impl<RepoErr> AgregarPreguntas<RepoErr> {
    pub fn new(reposotorio: Box<dyn RepositorioAgregarPregunta<RepoErr>>) -> Self {
        Self { reposotorio }
    }
}

#[async_trait]
impl<RepoErr> CasoDeUso<InputData, (), PreguntaError> for AgregarPreguntas<RepoErr>
where
    PreguntaError: From<RepoErr>,
{
    async fn ejecutar(&self, in_: InputData) -> Result<(), PreguntaError> {
        let examen_id = ExamenID::new(&in_.examen_id)?;
        let preguntas = in_
            .preguntas
            .into_iter()
            .map(|i| {
                PreguntaEntity::new(
                    i.id,
                    i.contenido,
                    i.etiqueta,
                    i.tipo_de_pregunta,
                    i.imagen_ref,
                    i.alternativas,
                    i.puntos,
                )
            })
            .collect::<Result<Vec<PreguntaEntity>, PreguntaError>>()?;
        let lista_de_preguntas = ListaDePreguntas::new(preguntas);
        self.reposotorio
            .agregar(examen_id, lista_de_preguntas)
            .await?;
        Ok(())
    }
}
