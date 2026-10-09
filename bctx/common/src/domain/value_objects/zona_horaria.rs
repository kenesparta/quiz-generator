use chrono::{DateTime, FixedOffset, NaiveDateTime, Utc};
use chrono_tz::America::Lima;

pub fn ahora_lima() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&Lima).fixed_offset()
}

pub fn utc_a_lima(dt: DateTime<Utc>) -> DateTime<FixedOffset> {
    dt.with_timezone(&Lima).fixed_offset()
}

/// Una fecha y hora sin zona interpretada como hora de Lima. `None` si no existe o es ambigua
/// en esa zona.
pub fn hora_de_lima(local: NaiveDateTime) -> Option<DateTime<FixedOffset>> {
    local
        .and_local_timezone(Lima)
        .single()
        .map(|fecha| fecha.fixed_offset())
}

pub fn formatear_rfc3339(dt: &DateTime<FixedOffset>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMA_OFFSET_SECONDS: i32 = -5 * 3600;

    #[test]
    fn test_ahora_lima_tiene_offset_menos_5() {
        let ahora = ahora_lima();
        let offset = ahora.offset().local_minus_utc();
        assert_eq!(offset, LIMA_OFFSET_SECONDS);
    }

    #[test]
    fn test_formato_rfc3339_contiene_offset() {
        let ahora = ahora_lima();
        let formatted = formatear_rfc3339(&ahora);
        assert!(formatted.contains("-05:00"));
        assert!(formatted.contains('T'));
    }

    #[test]
    fn test_utc_a_lima_convierte_correctamente() {
        let utc = Utc::now();
        let lima = utc_a_lima(utc);
        assert_eq!(lima.offset().local_minus_utc(), LIMA_OFFSET_SECONDS);
    }

    #[test]
    fn interpreta_una_hora_local_como_hora_de_lima() {
        let local =
            NaiveDateTime::parse_from_str("2026-01-05 10:00:00", "%Y-%m-%d %H:%M:%S").unwrap();
        let lima = hora_de_lima(local).unwrap();
        assert_eq!(lima.offset().local_minus_utc(), LIMA_OFFSET_SECONDS);
        assert_eq!(formatear_rfc3339(&lima), "2026-01-05T10:00:00.000000-05:00");
    }
}
