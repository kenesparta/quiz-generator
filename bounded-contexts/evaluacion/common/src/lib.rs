//! # Evaluación Common
//!
//! **Tipo de Subdominio:** Genérico
//!
//! Este crate proporciona tipos y utilidades compartidas que son reutilizables
//! a través de múltiples bounded contexts. Contiene abstracciones fundamentales
//! que no son específicas del dominio de evaluación.
//!
//! ## Módulos
//!
//! - [`entity`] - Trait base para entidades DDD con identidad única
//! - [`id`] - Identificador ULID como Value Object inmutable
//! - [`simple_name`] - Value Object para nombres validados con caracteres permitidos
//! - [`validator`] - Servicio de validación para operaciones comunes de strings
//!
//! ## Ejemplo
//!
//! ```
//! use common::{Id, Entity, SimpleName, Validator};
//!
//! // Crear un identificador único
//! let id = Id::new();
//!
//! // Validar un nombre
//! let nombre = SimpleName::new("Curso de Rust".to_string()).unwrap();
//! ```

mod entity;
mod id;
mod simple_name;
mod validator;

pub use entity::*;
pub use id::*;
pub use simple_name::*;
pub use validator::*;
