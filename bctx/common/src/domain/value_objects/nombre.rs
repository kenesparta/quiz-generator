use regex::Regex;
use std::sync::LazyLock;

/// Letras latinas (incluidas á, ü, ñ), apóstrofo y espacios simples. El rango anterior
/// (`À-ú`) rechazaba "ü" (Agüero, Sigüeñas), aceptaba los signos × y ÷, y `\s` dejaba pasar
/// tabuladores y saltos de línea.
static NOMBRE_REGEX: LazyLock<Regex> = LazyLock::new(|| {
    #[expect(clippy::expect_used, reason = "patrón literal, cubierto por los tests")]
    Regex::new(r"^[\p{Latin}']+( [\p{Latin}']+)*$").expect("patrón de nombres no válido")
});

pub fn nombre_regex() -> &'static Regex {
    &NOMBRE_REGEX
}

/// Quita los espacios de los extremos y deja uno solo entre palabras (también si eran
/// tabuladores o saltos de línea). Se aplica antes de validar: un espacio de más al escribir
/// un nombre no es motivo para rechazarlo.
pub fn normalizar_espacios(texto: &str) -> String {
    texto.split_whitespace().collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acepta_nombres_reales() {
        for nombre in [
            "Agüero",
            "Argüelles",
            "Sigüeñas",
            "D'Ángelo",
            "Ñahui",
            "De la Cruz",
            "María José",
        ] {
            assert!(nombre_regex().is_match(nombre), "{nombre}");
        }
    }

    #[test]
    fn normaliza_los_espacios_antes_de_validar() {
        for (escrito, normalizado) in [
            ("  María José ", "María José"),
            ("De  la   Cruz", "De la Cruz"),
            ("Juan\tPérez\n", "Juan Pérez"),
            ("   ", ""),
        ] {
            assert_eq!(normalizar_espacios(escrito), normalizado, "{escrito:?}");
        }
    }

    #[test]
    fn rechaza_simbolos_digitos_y_espacios_raros() {
        for nombre in [
            "Pérez×2",
            "Pérez÷",
            "Juan\nPérez",
            "Juan\tPérez",
            "Juan  Pérez",
            "R2D2",
            "",
        ] {
            assert!(!nombre_regex().is_match(nombre), "{nombre:?}");
        }
    }
}
