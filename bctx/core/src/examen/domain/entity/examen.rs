use crate::examen::domain::error::examen::ExamenError;
use crate::examen::domain::value_object::id::ExamenID;
use crate::pregunta::domain::service::tipo_pregunta::TipoDePregunta;
// use quizz_common::domain::entity::entidad::Entidad;

#[derive(Debug, Clone)]
pub struct Examen {
    id: ExamenID,
    titulo: String,
    descripcion: String,
    duracion_minutos: u32,
    puntos_totales: u32,
    categoria: String,
    nivel_dificultad: String,
    preguntas_ids: Vec<TipoDePregunta>,
    activo: bool,
}

// impl Entidad for Examen {
//     type ID = ExamenID;
//
//     fn id(&self) -> &Self::ID {
//         &self.id
//     }
// }

impl Examen {
    pub fn new(
        id: String,
        titulo: String,
        descripcion: String,
        duracion_minutos: u32,
        puntos_totales: u32,
        categoria: String,
        nivel_dificultad: String,
        preguntas_ids: Vec<TipoDePregunta>,
        activo: bool,
    ) -> Result<Self, ExamenError> {
        if titulo.trim().is_empty() {
            return Err(ExamenError::TituloInvalido);
        }

        if descripcion.trim().is_empty() {
            return Err(ExamenError::DescripcionInvalida);
        }

        if duracion_minutos < 10 || duracion_minutos > 240 {
            return Err(ExamenError::DuracionInvalida);
        }

        if puntos_totales == 0 {
            return Err(ExamenError::PuntosTotalesInvalidos);
        }

        if categoria.trim().is_empty() {
            return Err(ExamenError::CategoriaInvalida);
        }

        let examen_id = ExamenID::new(&id)?;

        Ok(Self {
            id: examen_id,
            titulo,
            descripcion,
            duracion_minutos,
            puntos_totales,
            categoria,
            nivel_dificultad,
            preguntas_ids,
            activo,
        })
    }

    pub fn titulo(&self) -> &str {
        &self.titulo
    }

    pub fn descripcion(&self) -> &str {
        &self.descripcion
    }

    pub fn duracion_minutos(&self) -> u32 {
        self.duracion_minutos
    }

    pub fn puntos_totales(&self) -> u32 {
        self.puntos_totales
    }

    pub fn categoria(&self) -> &str {
        &self.categoria
    }

    pub fn nivel_dificultad(&self) -> &str {
        &self.nivel_dificultad
    }

    pub fn preguntas_ids(&self) -> &Vec<String> {
        &self.preguntas_ids
    }

    pub fn activo(&self) -> bool {
        self.activo
    }

    pub fn activar(&mut self) {
        self.activo = true;
    }

    pub fn desactivar(&mut self) {
        self.activo = false;
    }
}
