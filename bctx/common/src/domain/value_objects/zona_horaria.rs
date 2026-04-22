use chrono::{DateTime, FixedOffset, Utc};
use chrono_tz::America::Lima;

const LIMA_OFFSET_SECONDS: i32 = -5 * 3600;

pub fn ahora_lima() -> DateTime<FixedOffset> {
    Utc::now().with_timezone(&Lima).fixed_offset()
}

pub fn utc_a_lima(dt: DateTime<Utc>) -> DateTime<FixedOffset> {
    dt.with_timezone(&Lima).fixed_offset()
}

pub fn offset_lima() -> FixedOffset {
    FixedOffset::west_opt(-LIMA_OFFSET_SECONDS).expect("offset Lima invalido")
}

pub fn formatear_rfc3339(dt: &DateTime<FixedOffset>) -> String {
    dt.to_rfc3339_opts(chrono::SecondsFormat::Micros, false)
}

#[cfg(test)]
mod tests {
    use super::*;

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
}
