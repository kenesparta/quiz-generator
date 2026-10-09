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
