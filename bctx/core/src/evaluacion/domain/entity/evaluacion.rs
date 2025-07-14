use crate::evaluacion::domain::error::evaluacion::EvaluacionError;
use crate::evaluacion::domain::value_object::evaluacion_estado::EvaluacionEstado;
use crate::evaluacion::value_object::id::EvaluacionID;
use crate::examen::domain::service::lista_examenes::ListaDeExamenes;
use quizz_common::domain::value_objects::estado::EstadoGeneral;

pub struct Evaluacion {
    pub id: EvaluacionID,
    pub nombre: String,
    pub descripcion: String,
    pub esta_activo: EstadoGeneral,
    pub estado: EvaluacionEstado,
    pub examenes: ListaDeExamenes,
}

impl Evaluacion {
    pub fn new(id: String, nombre: String, descripcion: String) -> Result<Self, EvaluacionError> {
        if nombre.trim().is_empty() {
            return Err(EvaluacionError::NombreNoValido);
        }

        if descripcion.trim().is_empty() {
            return Err(EvaluacionError::DescripcionNoValida);
        }

        let id = EvaluacionID::new(&id)?;
        let esta_activo = EstadoGeneral::default();
        let estado = EvaluacionEstado::default();

        Ok(Self {
            id,
            nombre,
            descripcion,
            esta_activo,
            estado,
            examenes: ListaDeExamenes::new(Vec::new()),
        })
    }

    pub fn publicar(&mut self) {
        self.estado = EvaluacionEstado::Publicado
    }

    pub fn esta_publicada(&self) -> bool {
        self.estado == EvaluacionEstado::Publicado
    }
}
