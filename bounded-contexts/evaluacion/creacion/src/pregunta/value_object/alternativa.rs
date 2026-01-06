use std::fmt;
use std::str::FromStr;
use thiserror::Error;

#[derive(Error, Debug, Clone, PartialEq, Eq)]
pub enum AlternativaError {
    #[error("Alternativa no válida: {0}")]
    NoValida(String),

    #[error("Alternativas vacías")]
    Vacias,

    #[error("Se requieren al menos {min} alternativas, se proporcionaron {actual}")]
    InsuficientesAlternativas { min: usize, actual: usize },

    #[error("Se permiten máximo {max} alternativas, se proporcionaron {actual}")]
    DemasiadasAlternativas { max: usize, actual: usize },

    #[error("Clave de alternativa duplicada: {0}")]
    ClaveDuplicada(String),
}

/// Value Object que representa una clave de alternativa válida.
///
/// Las claves válidas son: A, B, C, D, E, F, G (para alternativas múltiples)
/// y SI, NO (para preguntas de tipo si/no).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum AlternativaClave {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    Si,
    No,
}

impl AlternativaClave {
    /// Retorna true si es una clave de alternativa múltiple (A-G)
    #[must_use]
    pub const fn es_multiple(&self) -> bool {
        matches!(
            self,
            Self::A | Self::B | Self::C | Self::D | Self::E | Self::F | Self::G
        )
    }

    /// Retorna true si es una clave de si/no
    #[must_use]
    pub const fn es_si_no(&self) -> bool {
        matches!(self, Self::Si | Self::No)
    }

    /// Retorna todas las claves de alternativa múltiple
    #[must_use]
    pub const fn multiples() -> [Self; 7] {
        [
            Self::A,
            Self::B,
            Self::C,
            Self::D,
            Self::E,
            Self::F,
            Self::G,
        ]
    }

    /// Retorna las claves de si/no
    #[must_use]
    pub const fn si_no() -> [Self; 2] {
        [Self::Si, Self::No]
    }
}

impl fmt::Display for AlternativaClave {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::A => write!(f, "A"),
            Self::B => write!(f, "B"),
            Self::C => write!(f, "C"),
            Self::D => write!(f, "D"),
            Self::E => write!(f, "E"),
            Self::F => write!(f, "F"),
            Self::G => write!(f, "G"),
            Self::Si => write!(f, "SI"),
            Self::No => write!(f, "NO"),
        }
    }
}

impl FromStr for AlternativaClave {
    type Err = AlternativaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::A),
            "B" => Ok(Self::B),
            "C" => Ok(Self::C),
            "D" => Ok(Self::D),
            "E" => Ok(Self::E),
            "F" => Ok(Self::F),
            "G" => Ok(Self::G),
            "SI" => Ok(Self::Si),
            "NO" => Ok(Self::No),
            _ => Err(AlternativaError::NoValida(s.to_string())),
        }
    }
}

/// Value Object que representa una alternativa con su clave y texto.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alternativa {
    clave: AlternativaClave,
    texto: String,
}

impl Alternativa {
    /// Crea una nueva alternativa.
    #[must_use]
    pub fn new(clave: AlternativaClave, texto: String) -> Self {
        Self { clave, texto }
    }

    /// Crea una alternativa desde strings.
    ///
    /// # Errors
    ///
    /// Retorna error si la clave no es válida.
    pub fn from_strings(clave: &str, texto: String) -> Result<Self, AlternativaError> {
        let clave = clave.parse::<AlternativaClave>()?;
        Ok(Self::new(clave, texto))
    }

    #[must_use]
    pub fn clave(&self) -> AlternativaClave {
        self.clave
    }

    #[must_use]
    pub fn texto(&self) -> &str {
        &self.texto
    }
}

/// Value Object que representa un conjunto de alternativas múltiples (A-G).
///
/// Garantiza que:
/// - Hay al menos 2 alternativas
/// - No hay más de 7 alternativas
/// - No hay claves duplicadas
/// - Todas las claves son de tipo múltiple (A-G)
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativasMultiples {
    items: Vec<Alternativa>,
}

impl AlternativasMultiples {
    const MIN_ALTERNATIVAS: usize = 2;
    const MAX_ALTERNATIVAS: usize = 7;

    /// Crea un nuevo conjunto de alternativas múltiples.
    ///
    /// # Errors
    ///
    /// - `AlternativaError::Vacias` si el vector está vacío
    /// - `AlternativaError::InsuficientesAlternativas` si hay menos de 2
    /// - `AlternativaError::DemasiadasAlternativas` si hay más de 7
    /// - `AlternativaError::ClaveDuplicada` si hay claves repetidas
    /// - `AlternativaError::NoValida` si alguna clave no es de tipo múltiple
    pub fn new(items: Vec<Alternativa>) -> Result<Self, AlternativaError> {
        if items.is_empty() {
            return Err(AlternativaError::Vacias);
        }

        if items.len() < Self::MIN_ALTERNATIVAS {
            return Err(AlternativaError::InsuficientesAlternativas {
                min: Self::MIN_ALTERNATIVAS,
                actual: items.len(),
            });
        }

        if items.len() > Self::MAX_ALTERNATIVAS {
            return Err(AlternativaError::DemasiadasAlternativas {
                max: Self::MAX_ALTERNATIVAS,
                actual: items.len(),
            });
        }

        // Verificar que todas sean de tipo múltiple y no haya duplicados
        let mut seen = std::collections::HashSet::new();
        for item in &items {
            if !item.clave().es_multiple() {
                return Err(AlternativaError::NoValida(format!(
                    "{} no es una clave de alternativa múltiple",
                    item.clave()
                )));
            }
            if !seen.insert(item.clave()) {
                return Err(AlternativaError::ClaveDuplicada(item.clave().to_string()));
            }
        }

        Ok(Self { items })
    }

    #[must_use]
    pub fn items(&self) -> &[Alternativa] {
        &self.items
    }

    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// Busca una alternativa por su clave.
    #[must_use]
    pub fn buscar(&self, clave: AlternativaClave) -> Option<&Alternativa> {
        self.items.iter().find(|a| a.clave() == clave)
    }

    /// Verifica si contiene una clave específica.
    #[must_use]
    pub fn contiene(&self, clave: AlternativaClave) -> bool {
        self.items.iter().any(|a| a.clave() == clave)
    }
}

/// Value Object que representa las alternativas de una pregunta Si/No.
///
/// Siempre contiene exactamente dos alternativas: SI y NO.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AlternativasSiNo {
    si: String,
    no: String,
}

impl AlternativasSiNo {
    /// Crea nuevas alternativas Si/No.
    #[must_use]
    pub fn new(texto_si: String, texto_no: String) -> Self {
        Self {
            si: texto_si,
            no: texto_no,
        }
    }

    /// Crea alternativas Si/No con textos por defecto.
    #[must_use]
    pub fn default_texts() -> Self {
        Self {
            si: "Sí".to_string(),
            no: "No".to_string(),
        }
    }

    #[must_use]
    pub fn texto_si(&self) -> &str {
        &self.si
    }

    #[must_use]
    pub fn texto_no(&self) -> &str {
        &self.no
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod alternativa_clave_tests {
        use super::*;

        #[test]
        fn test_parse_valida() {
            assert_eq!(
                "A".parse::<AlternativaClave>().unwrap(),
                AlternativaClave::A
            );
            assert_eq!(
                "b".parse::<AlternativaClave>().unwrap(),
                AlternativaClave::B
            );
            assert_eq!(
                "SI".parse::<AlternativaClave>().unwrap(),
                AlternativaClave::Si
            );
            assert_eq!(
                "no".parse::<AlternativaClave>().unwrap(),
                AlternativaClave::No
            );
        }

        #[test]
        fn test_parse_invalida() {
            assert!("X".parse::<AlternativaClave>().is_err());
            assert!("".parse::<AlternativaClave>().is_err());
            assert!("AB".parse::<AlternativaClave>().is_err());
        }

        #[test]
        fn test_es_multiple() {
            assert!(AlternativaClave::A.es_multiple());
            assert!(AlternativaClave::G.es_multiple());
            assert!(!AlternativaClave::Si.es_multiple());
            assert!(!AlternativaClave::No.es_multiple());
        }

        #[test]
        fn test_es_si_no() {
            assert!(AlternativaClave::Si.es_si_no());
            assert!(AlternativaClave::No.es_si_no());
            assert!(!AlternativaClave::A.es_si_no());
        }
    }

    mod alternativas_multiples_tests {
        use super::*;

        fn crear_alternativas(claves: &[&str]) -> Vec<Alternativa> {
            claves
                .iter()
                .map(|c| Alternativa::from_strings(c, format!("Texto {}", c)).unwrap())
                .collect()
        }

        #[test]
        fn test_creacion_valida() {
            let items = crear_alternativas(&["A", "B", "C"]);
            let result = AlternativasMultiples::new(items);
            assert!(result.is_ok());
            assert_eq!(result.unwrap().len(), 3);
        }

        #[test]
        fn test_minimo_dos_alternativas() {
            let items = crear_alternativas(&["A"]);
            let result = AlternativasMultiples::new(items);
            assert!(matches!(
                result,
                Err(AlternativaError::InsuficientesAlternativas { .. })
            ));
        }

        #[test]
        fn test_maximo_siete_alternativas() {
            let items = crear_alternativas(&["A", "B", "C", "D", "E", "F", "G"]);
            let result = AlternativasMultiples::new(items);
            assert!(result.is_ok());
        }

        #[test]
        fn test_no_permite_claves_si_no() {
            let mut items = crear_alternativas(&["A", "B"]);
            items.push(Alternativa::new(AlternativaClave::Si, "Sí".to_string()));
            let result = AlternativasMultiples::new(items);
            assert!(matches!(result, Err(AlternativaError::NoValida(_))));
        }

        #[test]
        fn test_no_permite_duplicados() {
            let items = vec![
                Alternativa::new(AlternativaClave::A, "Texto 1".to_string()),
                Alternativa::new(AlternativaClave::A, "Texto 2".to_string()),
            ];
            let result = AlternativasMultiples::new(items);
            assert!(matches!(result, Err(AlternativaError::ClaveDuplicada(_))));
        }

        #[test]
        fn test_buscar_alternativa() {
            let items = crear_alternativas(&["A", "B", "C"]);
            let alternativas = AlternativasMultiples::new(items).unwrap();

            assert!(alternativas.buscar(AlternativaClave::A).is_some());
            assert!(alternativas.buscar(AlternativaClave::D).is_none());
        }
    }

    mod alternativas_si_no_tests {
        use super::*;

        #[test]
        fn test_creacion() {
            let alt = AlternativasSiNo::new("Verdadero".to_string(), "Falso".to_string());
            assert_eq!(alt.texto_si(), "Verdadero");
            assert_eq!(alt.texto_no(), "Falso");
        }

        #[test]
        fn test_default() {
            let alt = AlternativasSiNo::default_texts();
            assert_eq!(alt.texto_si(), "Sí");
            assert_eq!(alt.texto_no(), "No");
        }
    }
}
