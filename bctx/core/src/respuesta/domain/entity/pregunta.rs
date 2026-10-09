use crate::pregunta::domain::value_object::alternativa::Alternativa;
use crate::pregunta::domain::value_object::etiqueta::Etiqueta;
use crate::pregunta::domain::value_object::id::PreguntaID;
use crate::pregunta::domain::value_object::tipo_pregunta::TipoPregunta;
use crate::respuesta::domain::entity::respuesta::Estado;
use crate::respuesta::domain::error::respuesta::RespuestaError;
use std::collections::HashMap;

pub type Puntaje = HashMap<String, u32>;

/// Lo necesario para corregir una contestación: el estado de la hoja de respuestas y la
/// pregunta tal como quedó en su copia de la evaluación (incluida la clave de corrección).
pub struct PreguntaACorregir {
    pub estado: Estado,
    pub tipo_de_pregunta: TipoPregunta,
    pub alternativas: HashMap<String, String>,
    pub puntaje: Puntaje,
}

pub struct Pregunta {
    pub id: PreguntaID,
    pub contenido: String,
    pub observaciones: String,
    pub etiqueta: Etiqueta,
    pub tipo_de_pregunta: TipoPregunta,
    pub imagen_ref: String,
    pub alternativas: HashMap<String, String>,

    // puntaje se refiere al puntaje dado por pregunta
    pub puntaje: Puntaje,
    pub respuestas: Option<Vec<String>>,
    pub puntos: i64,
}

/// Máximo de caracteres de una respuesta escrita (preguntas `libre` y `sola_respuesta`).
pub const MAX_CARACTERES_RESPUESTA: usize = 2000;

/// Corrige la contestación de una pregunta y devuelve sus puntos.
///
/// Todos los tipos de pregunta admiten una sola respuesta: una lista vacía vale 0 (pregunta
/// sin contestar) y dos o más respuestas son un error, así repetir una clave o marcar todas las
/// alternativas no suma puntos. En las preguntas de alternativas la respuesta debe ser una de
/// sus claves (sin distinguir mayúsculas); en las escritas, un texto de hasta
/// [`MAX_CARACTERES_RESPUESTA`] caracteres que puntúa si coincide con una clave del puntaje.
///
/// # Errors
///
/// [`RespuestaError::CantidadDeRespuestasNoValida`] con más de una respuesta y
/// [`RespuestaError::RespuestaNoValida`] si la respuesta no es una alternativa de la pregunta
/// o es demasiado larga.
pub fn corregir_respuesta(
    tipo: &TipoPregunta,
    respuestas: &[String],
    alternativas: &HashMap<String, String>,
    puntaje: &Puntaje,
) -> Result<u32, RespuestaError> {
    let respuesta = match respuestas {
        [] => return Ok(0),
        [unica] => unica,
        _ => return Err(RespuestaError::CantidadDeRespuestasNoValida),
    };

    match tipo {
        TipoPregunta::AlternativaUnica | TipoPregunta::AlternativaConPeso | TipoPregunta::SioNo => {
            let es_alternativa = if alternativas.is_empty() && matches!(tipo, TipoPregunta::SioNo) {
                // Una pregunta de sí o no puede no listar sus dos alternativas.
                respuesta
                    .parse::<Alternativa>()
                    .is_ok_and(|a| matches!(a, Alternativa::Si | Alternativa::No))
            } else {
                alternativas
                    .keys()
                    .any(|clave| clave.eq_ignore_ascii_case(respuesta))
            };
            if !es_alternativa {
                return Err(RespuestaError::RespuestaNoValida);
            }
            Ok(puntaje
                .iter()
                .find(|(clave, _)| clave.eq_ignore_ascii_case(respuesta))
                .map_or(0, |(_, puntos)| *puntos))
        }
        TipoPregunta::Libre | TipoPregunta::SolaRespuesta => {
            if respuesta.chars().count() > MAX_CARACTERES_RESPUESTA {
                return Err(RespuestaError::RespuestaNoValida);
            }
            Ok(puntaje.get(respuesta).copied().unwrap_or(0))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn alternativas(claves: &[&str]) -> HashMap<String, String> {
        claves
            .iter()
            .map(|c| ((*c).to_string(), format!("texto {c}")))
            .collect()
    }

    fn puntaje(pares: &[(&str, u32)]) -> Puntaje {
        pares.iter().map(|(c, p)| ((*c).to_string(), *p)).collect()
    }

    fn respuestas(r: &[&str]) -> Vec<String> {
        r.iter().map(|s| (*s).to_string()).collect()
    }

    #[test]
    fn la_alternativa_correcta_suma_sus_puntos() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaUnica,
            &respuestas(&["A"]),
            &alternativas(&["A", "B"]),
            &puntaje(&[("A", 1)]),
        );
        assert_eq!(resultado.unwrap(), 1);
    }

    #[test]
    fn una_alternativa_incorrecta_vale_cero() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaUnica,
            &respuestas(&["B"]),
            &alternativas(&["A", "B"]),
            &puntaje(&[("A", 1)]),
        );
        assert_eq!(resultado.unwrap(), 0);
    }

    #[test]
    fn en_las_preguntas_con_peso_cada_alternativa_vale_lo_suyo() {
        let pesos = puntaje(&[("A", 0), ("B", 4), ("C", 2)]);
        for (respuesta, esperado) in [("A", 0), ("B", 4), ("c", 2)] {
            let resultado = corregir_respuesta(
                &TipoPregunta::AlternativaConPeso,
                &respuestas(&[respuesta]),
                &alternativas(&["A", "B", "C"]),
                &pesos,
            );
            assert_eq!(resultado.unwrap(), esperado, "{respuesta}");
        }
    }

    #[test]
    fn repetir_la_clave_correcta_no_multiplica_los_puntos() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaUnica,
            &respuestas(&["B", "B", "B"]),
            &alternativas(&["A", "B"]),
            &puntaje(&[("B", 4)]),
        );
        assert!(matches!(
            resultado,
            Err(RespuestaError::CantidadDeRespuestasNoValida)
        ));
    }

    #[test]
    fn marcar_todas_las_alternativas_no_puntua() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaConPeso,
            &respuestas(&["A", "B", "C"]),
            &alternativas(&["A", "B", "C"]),
            &puntaje(&[("A", 1), ("B", 2), ("C", 3)]),
        );
        assert!(matches!(
            resultado,
            Err(RespuestaError::CantidadDeRespuestasNoValida)
        ));
    }

    #[test]
    fn una_clave_que_no_es_alternativa_se_rechaza() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaUnica,
            &respuestas(&["Z"]),
            &alternativas(&["A", "B"]),
            &puntaje(&[("A", 1)]),
        );
        assert!(matches!(resultado, Err(RespuestaError::RespuestaNoValida)));
    }

    #[test]
    fn sin_respuesta_vale_cero() {
        let resultado = corregir_respuesta(
            &TipoPregunta::AlternativaUnica,
            &[],
            &alternativas(&["A", "B"]),
            &puntaje(&[("A", 1)]),
        );
        assert_eq!(resultado.unwrap(), 0);
    }

    #[test]
    fn si_o_no_sin_alternativas_acepta_si_y_no() {
        let pesos = puntaje(&[("SI", 1)]);
        let si = corregir_respuesta(
            &TipoPregunta::SioNo,
            &respuestas(&["si"]),
            &HashMap::new(),
            &pesos,
        );
        assert_eq!(si.unwrap(), 1);
        let otra = corregir_respuesta(
            &TipoPregunta::SioNo,
            &respuestas(&["A"]),
            &HashMap::new(),
            &pesos,
        );
        assert!(matches!(otra, Err(RespuestaError::RespuestaNoValida)));
    }

    #[test]
    fn una_respuesta_escrita_puntua_si_coincide_y_tiene_un_limite() {
        let clave = puntaje(&[("Lima", 2)]);
        let correcta = corregir_respuesta(
            &TipoPregunta::SolaRespuesta,
            &respuestas(&["Lima"]),
            &HashMap::new(),
            &clave,
        );
        assert_eq!(correcta.unwrap(), 2);

        let en_el_limite = "x".repeat(MAX_CARACTERES_RESPUESTA);
        let libre = corregir_respuesta(
            &TipoPregunta::Libre,
            &[en_el_limite],
            &HashMap::new(),
            &Puntaje::new(),
        );
        assert_eq!(libre.unwrap(), 0);

        let larga = "ñ".repeat(MAX_CARACTERES_RESPUESTA + 1);
        let resultado = corregir_respuesta(
            &TipoPregunta::Libre,
            &[larga],
            &HashMap::new(),
            &Puntaje::new(),
        );
        assert!(matches!(resultado, Err(RespuestaError::RespuestaNoValida)));
    }
}
