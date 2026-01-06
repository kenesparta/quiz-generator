//! # Evaluación Creación
//!
//! **Tipo de Subdominio:** Core (Principal)
//!
//! Este crate contiene la lógica de dominio principal para la creación y gestión
//! de evaluaciones. Es el corazón del sistema y representa la ventaja competitiva
//! del negocio.
//!
//! ## Conceptos de Dominio
//!
//! - **Examen**: Colección de preguntas agrupadas temáticamente
//! - **Pregunta**: Ítem individual con diferentes tipos de respuesta y puntuación
//! - **Evaluación**: Composición de uno o más exámenes para asignar a candidatos
//!
//! ## Módulos
//!
//! - [`examen`] - Entidad y lógica para gestión de exámenes
//! - [`pregunta`] - Tipos de preguntas con estrategias de validación y puntuación
//! - [`evaluacion`] - Composición de exámenes en evaluaciones completas
//!
//! ## Patrones Utilizados
//!
//! - **Strategy Pattern**: Diferentes tipos de preguntas (alternativa única, sí/no, libre, etc.)
//! - **Value Objects**: Puntajes, etiquetas, alternativas como objetos inmutables
//! - **Aggregate Root**: Examen como raíz de agregado que contiene preguntas

mod evaluacion;
mod examen;
mod pregunta;

pub use examen::*;
pub use pregunta::*;
