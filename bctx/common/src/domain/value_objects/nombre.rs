use regex::Regex;
use std::sync::OnceLock;

static NOMBRE_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn nombre_regex() -> &'static Regex {
    NOMBRE_REGEX.get_or_init(|| {
        Regex::new(r"^[a-zA-ZÀ-ú'\s]+$").expect("Regex pattern for Spanish names is invalid")
    })
}
