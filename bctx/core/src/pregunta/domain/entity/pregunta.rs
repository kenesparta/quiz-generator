use crate::pregunta::domain::error::pregunta::PreguntaError;
use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use std::collections::HashMap;
use std::fmt::Debug;
use std::str::FromStr;

#[derive(Debug, Clone)]
pub struct PreguntaEntity {
    pub id: PreguntaID,
    pub contenido: String,
    pub imagen_ref: Option<String>,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: HashMap<String, String>,
    pub puntaje: HashMap<String, u32>,
}

impl PreguntaEntity {
    pub fn new(
        contenido: String,
        etiqueta: String,
        tipo_de_pregunta: String,
        imagen_ref: Option<String>,
        alternativas: HashMap<String, String>,
        puntaje: HashMap<String, u32>,
    ) -> Result<Self, PreguntaError> {
        let id = PreguntaID::new_v4();
        let etiqueta = Etiqueta::from_str(&etiqueta)?;
        let tipo_de_pregunta = TipoPregunta::from_str(&tipo_de_pregunta)?;

        validar_composicion(&tipo_de_pregunta, &alternativas, &puntaje)?;
        let imagen_ref = validar_imagen(imagen_ref)?;

        Ok(Self {
            id,
            contenido,
            etiqueta,
            tipo_de_pregunta,
            alternativas,
            puntaje,
            imagen_ref,
        })
    }
}

/// Tamaño máximo de la imagen de una pregunta (el data URI completo). La imagen se copia en
/// cada evaluación publicada y en cada hoja de respuestas, y MongoDB limita un documento a
/// 16 MB.
pub const MAX_BYTES_IMAGEN: usize = 512 * 1024;
const PREFIJOS_IMAGEN: [&str; 3] = [
    "data:image/png;base64,",
    "data:image/jpeg;base64,",
    "data:image/webp;base64,",
];

/// La imagen es opcional ("" equivale a no tener) y, si está, debe ser un data URI PNG, JPEG o
/// WebP de hasta [`MAX_BYTES_IMAGEN`]: se entrega tal cual a postulantes y psicólogos, así que
/// no se aceptan `javascript:`, HTML ni SVG.
fn validar_imagen(imagen: Option<String>) -> Result<Option<String>, PreguntaError> {
    match imagen.filter(|imagen| !imagen.is_empty()) {
        None => Ok(None),
        Some(imagen)
            if imagen.len() <= MAX_BYTES_IMAGEN
                && PREFIJOS_IMAGEN.iter().any(|p| imagen.starts_with(p)) =>
        {
            Ok(Some(imagen))
        }
        Some(_) => Err(PreguntaError::ImagenNoValida),
    }
}

/// Las reglas de alternativas y puntaje de cada tipo de pregunta, en un solo `match`.
///
/// - `alternativa_unica` y `alternativa_peso`: alternativas y puntaje no vacíos, con claves
///   A..G (o SI/NO), y cada clave del puntaje debe ser una alternativa.
/// - `sola_respuesta`: exactamente una clave en el puntaje (la respuesta correcta).
/// - `si_o_no`: si lista alternativas o puntaje, deben tener las claves SI y NO; el puntaje
///   solo puede usar claves que estén entre las alternativas.
/// - `libre`: sin reglas.
fn validar_composicion(
    tipo: &TipoPregunta,
    alternativas: &HashMap<String, String>,
    puntaje: &HashMap<String, u32>,
) -> Result<(), PreguntaError> {
    match tipo {
        TipoPregunta::AlternativaUnica | TipoPregunta::AlternativaConPeso => {
            if claves(alternativas)?.is_empty() {
                return Err(PreguntaError::AlternativasNoExisten);
            }
            if claves(puntaje)?.is_empty() {
                return Err(PreguntaError::PuntajeNoExiste);
            }
            puntaje_dentro_de_alternativas(alternativas, puntaje)
        }
        TipoPregunta::SolaRespuesta => match puntaje.len() {
            0 => Err(PreguntaError::PuntajeNoExiste),
            1 => Ok(()),
            _ => Err(PreguntaError::DebeTenerUnaSolaRespuesta),
        },
        TipoPregunta::SioNo => {
            exige_si_y_no(&claves(alternativas)?)?;
            exige_si_y_no(&claves(puntaje)?)?;
            puntaje_dentro_de_alternativas(alternativas, puntaje)
        }
        TipoPregunta::Libre => Ok(()),
    }
}

/// Las claves como alternativas; una clave que no es A..G, SI o NO es un error.
fn claves<V>(mapa: &HashMap<String, V>) -> Result<Vec<Alternativa>, PreguntaError> {
    mapa.keys()
        .map(|clave| clave.parse::<Alternativa>().map_err(PreguntaError::from))
        .collect()
}

/// Vacío o con SI y NO (otras claves se ignoran).
fn exige_si_y_no(claves: &[Alternativa]) -> Result<(), PreguntaError> {
    if claves.is_empty() || (claves.contains(&Alternativa::Si) && claves.contains(&Alternativa::No))
    {
        Ok(())
    } else {
        Err(PreguntaError::AlternativaNoAjustada)
    }
}

fn puntaje_dentro_de_alternativas(
    alternativas: &HashMap<String, String>,
    puntaje: &HashMap<String, u32>,
) -> Result<(), PreguntaError> {
    if puntaje.keys().all(|clave| alternativas.contains_key(clave)) {
        Ok(())
    } else {
        Err(PreguntaError::PuntajeNoCoincideConAlternativa)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pregunta::domain::value_object::alternativa::Alternativa;
    use std::collections::HashMap;

    #[test]
    fn test_create_alternativa_unica_question() {
        let contenido = "¿Cuál es la capital de Francia?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_unica".to_string();
        let imagen_ref = Some("".to_string());

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "París".to_string());
        alternativas.insert("B".to_string(), "Londres".to_string());
        alternativas.insert("C".to_string(), "Madrid".to_string());
        alternativas.insert("D".to_string(), "Roma".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 4);

        let result = PreguntaEntity::new(
            contenido.clone(),
            etiqueta.clone(),
            tipo_de_pregunta.clone(),
            imagen_ref.clone(),
            alternativas.clone(),
            puntaje.clone(),
        );

        assert!(result.is_ok());
        let pregunta = result.unwrap();
        assert_eq!(pregunta.contenido, contenido);
        assert_eq!(pregunta.etiqueta.to_string(), etiqueta);
        assert_eq!(pregunta.tipo_de_pregunta.to_string(), tipo_de_pregunta);

        let alt_map = pregunta.alternativas;
        let punt_map = pregunta.puntaje;

        assert!(alt_map.contains_key(&Alternativa::A.to_string()));
        assert!(alt_map.contains_key(&Alternativa::B.to_string()));
        assert!(alt_map.contains_key(&Alternativa::C.to_string()));
        assert!(alt_map.contains_key(&Alternativa::D.to_string()));

        assert_eq!(alt_map.get(&Alternativa::A.to_string()).unwrap(), "París");
        assert_eq!(punt_map.get(&Alternativa::A.to_string()).unwrap(), &4);
    }

    #[test]
    fn test_create_alternativa_peso_question() {
        let contenido = "¿Elige una de las alternativas?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_peso".to_string();
        let imagen_ref = Some("".to_string());

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "alt01".to_string());
        alternativas.insert("B".to_string(), "alt02".to_string());
        alternativas.insert("C".to_string(), "alt03".to_string());
        alternativas.insert("D".to_string(), "alt04".to_string());
        alternativas.insert("E".to_string(), "alt05".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 0);
        puntaje.insert("B".to_string(), 1);
        puntaje.insert("C".to_string(), 2);
        puntaje.insert("D".to_string(), 3);
        puntaje.insert("E".to_string(), 4);

        let result = PreguntaEntity::new(
            contenido.clone(),
            etiqueta.clone(),
            tipo_de_pregunta.clone(),
            imagen_ref.clone(),
            alternativas.clone(),
            puntaje.clone(),
        );

        assert!(result.is_ok());
        let pregunta = result.unwrap();
        assert_eq!(pregunta.contenido, contenido);
        assert_eq!(pregunta.tipo_de_pregunta.to_string(), tipo_de_pregunta);

        let alt_map = pregunta.alternativas;
        let punt_map = pregunta.puntaje;

        assert!(alt_map.contains_key(&Alternativa::A.to_string()));
        assert!(alt_map.contains_key(&Alternativa::B.to_string()));
        assert!(alt_map.contains_key(&Alternativa::C.to_string()));
        assert!(alt_map.contains_key(&Alternativa::D.to_string()));
        assert!(alt_map.contains_key(&Alternativa::E.to_string()));

        assert_eq!(punt_map.get(&Alternativa::A.to_string()).unwrap(), &0);
        assert_eq!(punt_map.get(&Alternativa::B.to_string()).unwrap(), &1);
        assert_eq!(punt_map.get(&Alternativa::C.to_string()).unwrap(), &2);
        assert_eq!(punt_map.get(&Alternativa::D.to_string()).unwrap(), &3);
        assert_eq!(punt_map.get(&Alternativa::E.to_string()).unwrap(), &4);
    }

    #[test]
    fn test_invalid_alternative_key() {
        let contenido = "¿Pregunta de prueba?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "alternativa_unica".to_string();
        let imagen_ref = None;

        let mut alternativas = HashMap::new();
        alternativas.insert("A".to_string(), "Opción A".to_string());
        alternativas.insert("X".to_string(), "Opción inválida".to_string());

        let mut puntaje = HashMap::new();
        puntaje.insert("A".to_string(), 1);

        let result = PreguntaEntity::new(
            contenido,
            etiqueta,
            tipo_de_pregunta,
            imagen_ref,
            alternativas,
            puntaje,
        );

        assert!(matches!(
            result,
            Err(PreguntaError::PreguntaAlternativaError(_))
        ));
    }

    #[test]
    fn test_invalid_tipo_pregunta() {
        let contenido = "¿Pregunta de prueba?".to_string();
        let etiqueta = "no".to_string();
        let tipo_de_pregunta = "tipo_invalido".to_string();
        let imagen_ref = None;

        let alternativas = HashMap::new();
        let puntaje = HashMap::new();

        let result = PreguntaEntity::new(
            contenido,
            etiqueta,
            tipo_de_pregunta,
            imagen_ref,
            alternativas,
            puntaje,
        );

        assert!(matches!(
            result,
            Err(PreguntaError::PreguntaTipoPreguntaError(_))
        ));
    }

    fn mapa<V: Clone>(pares: &[(&str, V)]) -> HashMap<String, V> {
        pares
            .iter()
            .map(|(k, v)| ((*k).to_string(), v.clone()))
            .collect()
    }

    fn validar(
        tipo: &str,
        alternativas: &[(&str, &str)],
        puntaje: &[(&str, u32)],
    ) -> Result<(), PreguntaError> {
        let alternativas: HashMap<String, String> = alternativas
            .iter()
            .map(|(k, v)| ((*k).to_string(), (*v).to_string()))
            .collect();
        validar_composicion(&tipo.parse().unwrap(), &alternativas, &mapa(puntaje))
    }

    #[test]
    fn alternativas_exigen_alternativas_y_puntaje_coherentes() {
        for tipo in ["alternativa_unica", "alternativa_peso"] {
            assert!(validar(tipo, &[("A", "x"), ("B", "y")], &[("A", 1)]).is_ok());
            assert!(matches!(
                validar(tipo, &[], &[("A", 1)]),
                Err(PreguntaError::AlternativasNoExisten)
            ));
            assert!(matches!(
                validar(tipo, &[("A", "x")], &[]),
                Err(PreguntaError::PuntajeNoExiste)
            ));
            assert!(matches!(
                validar(tipo, &[("A", "x")], &[("B", 1)]),
                Err(PreguntaError::PuntajeNoCoincideConAlternativa)
            ));
            assert!(matches!(
                validar(tipo, &[("Z", "x")], &[("Z", 1)]),
                Err(PreguntaError::PreguntaAlternativaError(_))
            ));
        }
    }

    #[test]
    fn sola_respuesta_exige_una_clave_en_el_puntaje() {
        assert!(validar("sola_respuesta", &[], &[("Lima", 1)]).is_ok());
        assert!(matches!(
            validar("sola_respuesta", &[], &[]),
            Err(PreguntaError::PuntajeNoExiste)
        ));
        assert!(matches!(
            validar("sola_respuesta", &[], &[("Lima", 1), ("Cusco", 1)]),
            Err(PreguntaError::DebeTenerUnaSolaRespuesta)
        ));
    }

    #[test]
    fn si_o_no_exige_si_y_no_cuando_las_lista() {
        assert!(validar("si_o_no", &[], &[]).is_ok());
        assert!(
            validar(
                "si_o_no",
                &[("SI", "Sí"), ("NO", "No")],
                &[("SI", 1), ("NO", 0)]
            )
            .is_ok()
        );
        assert!(matches!(
            validar("si_o_no", &[("SI", "Sí")], &[]),
            Err(PreguntaError::AlternativaNoAjustada)
        ));
        assert!(matches!(
            validar("si_o_no", &[("SI", "Sí"), ("NO", "No")], &[("SI", 1)]),
            Err(PreguntaError::AlternativaNoAjustada)
        ));
        assert!(matches!(
            validar("si_o_no", &[], &[("SI", 1), ("NO", 0)]),
            Err(PreguntaError::PuntajeNoCoincideConAlternativa)
        ));
    }

    #[test]
    fn libre_no_tiene_reglas() {
        assert!(validar("libre", &[("cualquier", "cosa")], &[("x", 3)]).is_ok());
    }

    #[test]
    fn la_imagen_es_opcional_y_vacia_equivale_a_ninguna() {
        assert_eq!(validar_imagen(None).unwrap(), None);
        assert_eq!(validar_imagen(Some(String::new())).unwrap(), None);
    }

    #[test]
    fn solo_se_aceptan_imagenes_png_jpeg_o_webp_de_tamano_acotado() {
        for valida in [
            "data:image/png;base64,iVBORw0KGgo=",
            "data:image/jpeg;base64,/9j/4AAQ",
            "data:image/webp;base64,UklGRg==",
        ] {
            assert!(validar_imagen(Some(valida.to_string())).is_ok(), "{valida}");
        }
        for no_valida in [
            "javascript:alert(1)",
            "data:text/html;base64,PHNjcmlwdD4=",
            "data:image/svg+xml;base64,PHN2Zz4=",
            "https://ejemplo.pe/imagen.png",
        ] {
            assert!(
                matches!(
                    validar_imagen(Some(no_valida.to_string())),
                    Err(PreguntaError::ImagenNoValida)
                ),
                "{no_valida}"
            );
        }
        let prefijo = "data:image/png;base64,";
        let justa = format!("{prefijo}{}", "A".repeat(MAX_BYTES_IMAGEN - prefijo.len()));
        assert!(validar_imagen(Some(justa.clone())).is_ok());
        assert!(matches!(
            validar_imagen(Some(format!("{justa}A"))),
            Err(PreguntaError::ImagenNoValida)
        ));
    }
}
