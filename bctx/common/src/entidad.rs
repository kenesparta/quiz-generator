use std::fmt::Debug;
use std::hash::{Hash, Hasher};
use uuid::Uuid;

/// Trait que debe implementar toda entidad en el sistema
pub trait Entidad: Clone + Debug + PartialEq + Eq + Hash {
    /// Retorna el identificador único de la entidad
    fn id(&self) -> &Uuid;

    /// Determina si esta entidad es igual a otra basándose solo en su identidad
    fn es_igual_a<E: Entidad>(&self, otra: &E) -> bool {
        self.id() == otra.id()
    }

    /// Determina si esta entidad es diferente de otra basándose solo en su identidad
    fn es_diferente_de<E: Entidad>(&self, otra: &E) -> bool {
        !self.es_igual_a(otra)
    }
}

/// Estructura base que puede ser utilizada para implementar entidades concretas
#[derive(Debug, Clone)]
pub struct EntidadBase {
    id: Uuid,
}

impl EntidadBase {
    /// Crea una nueva entidad con un ID generado aleatoriamente
    pub fn nueva() -> Self {
        Self { id: Uuid::new_v4() }
    }

    /// Crea una entidad con un ID específico (útil para reconstruir entidades desde almacenamiento)
    pub fn con_id(id: Uuid) -> Self {
        Self { id }
    }
}

impl PartialEq for EntidadBase {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for EntidadBase {}

impl Hash for EntidadBase {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl Entidad for EntidadBase {
    fn id(&self) -> &Uuid {
        &self.id
    }
}

/// Macro de utilidad para implementar Entidad en tipos concretos
#[macro_export]
macro_rules! implementa_entidad {
    ($tipo:ty) => {
        impl Entidad for $tipo {
            fn id(&self) -> &Uuid {
                &self.id
            }
        }

        impl PartialEq for $tipo {
            fn eq(&self, other: &Self) -> bool {
                self.id == other.id
            }
        }

        impl Eq for $tipo {}

        impl Hash for $tipo {
            fn hash<H: Hasher>(&self, state: &mut H) {
                self.id.hash(state);
            }
        }
    };
}

// Ejemplo de uso:
#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Usuario {
        id: Uuid,
        nombre: String,
        email: String,
    }

    implementa_entidad!(Usuario);

    impl Usuario {
        pub fn nuevo(nombre: String, email: String) -> Self {
            Self {
                id: Uuid::new_v4(),
                nombre,
                email,
            }
        }

        pub fn con_id(id: Uuid, nombre: String, email: String) -> Self {
            Self { id, nombre, email }
        }
    }

    #[test]
    fn test_igualdad_por_identidad() {
        let id = Uuid::new_v4();
        let usuario1 = Usuario::con_id(id, "Juan".to_string(), "juan@ejemplo.com".to_string());
        let usuario2 = Usuario::con_id(
            id,
            "Juanito".to_string(),
            "juanito@diferente.com".to_string(),
        );

        assert_eq!(usuario1, usuario2); // Son iguales porque tienen el mismo ID
        assert!(usuario1.es_igual_a(&usuario2));
        assert!(!usuario1.es_diferente_de(&usuario2));
    }

    #[test]
    fn test_desigualdad_por_identidad() {
        let usuario1 = Usuario::nuevo("Juan".to_string(), "juan@ejemplo.com".to_string());
        let usuario2 = Usuario::nuevo("Juan".to_string(), "juan@ejemplo.com".to_string());

        assert_ne!(usuario1, usuario2); // Son diferentes a pesar de tener los mismos datos
        assert!(!usuario1.es_igual_a(&usuario2));
        assert!(usuario1.es_diferente_de(&usuario2));
    }
}
