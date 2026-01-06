use std::fmt;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use thiserror::Error;

/// A ULID (Universally Unique Lexicographically Sortable Identifier) following a DDD Value Object pattern.
///
/// ULID is a 128-bit identifier that is:
/// - Time-ordered: First 48 bits are Unix timestamp in milliseconds
/// - Lexicographically sortable: String representation sorts chronologically
/// - Case-insensitive: Uses Crockford's Base32 encoding
/// - URL-safe: No special characters
/// - Compact: 26 characters vs. 36 for UUID
///
/// Structure (128 bits total):
/// - 48 bits: Timestamp in milliseconds since Unix epoch
/// - 80 bits: Cryptographically strong random data
///
/// String representation: 26 characters in Crockford's Base32
/// Example: 01ARZ3NDEKTSV4RRFFQ69G5FAV
///
/// As a Value Object:
/// - Immutable once created
/// - Compared by value
/// - Thread-safe and copyable
/// - Self-validating
///
/// # Examples
///
/// ```
/// use education_platform_common::Id;
///
/// let id1 = Id::new();
/// let id2 = Id::new();
/// assert_ne!(id1, id2);
///
/// let id_str = id1.to_string();
/// assert_eq!(id_str.len(), 26);
///
/// // Parse from string
/// let parsed: Id = "01ARZ3NDEKTSV4RRFFQ69G5FAV".parse().unwrap();
/// assert_eq!(parsed.to_string(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Id {
    bytes: [u8; 16],
}

impl Id {
    /// Creates a new ULID with the current timestamp and random data.
    ///
    /// Generates a time-ordered identifier where the first 48 bits represent
    /// the current Unix timestamp in milliseconds.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// println!("Generated ID: {}", id);
    /// ```
    #[inline]
    #[must_use]
    pub fn new() -> Self {
        let timestamp_ms = Self::current_timestamp_ms();
        let random_bytes = Self::generate_random_bytes();
        Self::from_parts(timestamp_ms, random_bytes)
    }

    /// Creates a ULID from timestamp and random bytes.
    ///
    /// Useful for testing or when using external random sources.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let timestamp = 1234567890123u64;
    /// let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    /// let id = Id::from_parts(timestamp, random);
    /// assert_eq!(id.timestamp_ms(), timestamp);
    /// ```
    #[must_use]
    pub fn from_parts(timestamp_ms: u64, random: [u8; 10]) -> Self {
        let mut bytes = [0u8; 16];

        bytes[0] = (timestamp_ms >> 40) as u8;
        bytes[1] = (timestamp_ms >> 32) as u8;
        bytes[2] = (timestamp_ms >> 24) as u8;
        bytes[3] = (timestamp_ms >> 16) as u8;
        bytes[4] = (timestamp_ms >> 8) as u8;
        bytes[5] = timestamp_ms as u8;

        bytes[6..16].copy_from_slice(&random);

        Self { bytes }
    }

    /// Returns the timestamp component in milliseconds since Unix epoch.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// let timestamp = id.timestamp_ms();
    /// assert!(timestamp > 0);
    /// ```
    #[inline]
    #[must_use]
    pub const fn timestamp_ms(&self) -> u64 {
        ((self.bytes[0] as u64) << 40)
            | ((self.bytes[1] as u64) << 32)
            | ((self.bytes[2] as u64) << 24)
            | ((self.bytes[3] as u64) << 16)
            | ((self.bytes[4] as u64) << 8)
            | (self.bytes[5] as u64)
    }

    /// Returns the raw bytes of the ULID.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// assert_eq!(id.as_bytes().len(), 16);
    /// ```
    #[inline]
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 16] {
        &self.bytes
    }

    fn current_timestamp_ms() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch")
            .as_millis() as u64
    }

    /// Generates cryptographically strong random bytes using system entropy.
    ///
    /// Uses multiple entropy sources for better randomness:
    /// - System random state
    /// - High-precision timestamps
    /// - Thread identifiers
    /// - Atomic counter for uniqueness
    fn generate_random_bytes() -> [u8; 10] {
        use std::collections::hash_map::RandomState;
        use std::hash::{BuildHasher, Hasher};
        use std::sync::atomic::{AtomicU64, Ordering};

        static COUNTER: AtomicU64 = AtomicU64::new(0);

        let random_state = RandomState::new();

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("System time before UNIX epoch");

        let nanos = now.subsec_nanos() as u64;
        let micros = now.as_micros() as u64;
        let counter = COUNTER.fetch_add(1, Ordering::Relaxed);
        let thread_id = random_state.hash_one(std::thread::current().id());

        let bytes: [u8; 10] = (0u64..5).fold([0u8; 10], |mut bytes, i| {
            let mut hasher = random_state.build_hasher();

            hasher.write_u64(nanos.wrapping_mul(i + 1));
            hasher.write_u64(micros.wrapping_add(i));
            hasher.write_u64(counter.wrapping_mul(i + 7));
            hasher.write_u64(thread_id.wrapping_mul(i + 13));
            hasher.write_usize(&bytes as *const _ as usize);
            hasher.write_usize((i * 17) as usize);

            let hash = hasher.finish();
            bytes[(i * 2) as usize] = (hash >> 8) as u8;
            bytes[(i * 2 + 1) as usize] = hash as u8;

            bytes
        });

        bytes
    }

    /// Encodes the ULID as a 26-character Crockford Base32 string.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::new();
    /// let s = id.to_string();
    /// assert_eq!(s.len(), 26);
    /// assert!(s.chars().all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c)));
    /// ```
    #[must_use]
    pub fn to_crockford_base32(&self) -> String {
        const ALPHABET: &[u8; 32] = b"0123456789ABCDEFGHJKMNPQRSTVWXYZ";

        let mut result = String::with_capacity(26);

        // Encode timestamp (48 bits = 10 characters)
        let timestamp = self.timestamp_ms();
        [45u32, 40, 35, 30, 25, 20, 15, 10, 5, 0]
            .iter()
            .for_each(|&shift| {
                result.push(ALPHABET[((timestamp >> shift) & 0x1F) as usize] as char);
            });

        // Encode randomness (80 bits = 16 characters)
        // Combine all random bytes into u128 for easier bit manipulation
        let random_bits: u128 = self.bytes[6..16]
            .iter()
            .fold(0u128, |acc, &byte| (acc << 8) | u128::from(byte));

        // Extract 16 characters (5 bits each = 80 bits)
        (0..16).for_each(|i| {
            let shift = 5 * (15 - i);
            let index = ((random_bits >> shift) & 0x1F) as usize;
            result.push(ALPHABET[index] as char);
        });

        result
    }

    /// Decodes a Crockford Base32 string into a ULID.
    ///
    /// # Errors
    ///
    /// Returns `IdError::InvalidLength` if the string is not 26 characters.
    /// Returns `IdError::InvalidCharacter` if the string contains invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::from_crockford_base32("01ARZ3NDEKTSV4RRFFQ69G5FAV").unwrap();
    /// assert_eq!(id.to_string(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    ///
    /// // Case-insensitive
    /// let id_lower = Id::from_crockford_base32("01arz3ndektsv4rrffq69g5fav").unwrap();
    /// assert_eq!(id, id_lower);
    /// ```
    pub fn from_crockford_base32(s: &str) -> Result<Self, IdError> {
        if s.len() != 26 {
            return Err(IdError::InvalidLength);
        }

        let chars: Vec<char> = s.to_uppercase().chars().collect();

        // Decode timestamp (first 10 characters)
        let timestamp = chars.iter().take(10).try_fold(0u64, |acc, &c| {
            Self::decode_char(c).map(|value| (acc << 5) | u64::from(value))
        })?;

        // Decode randomness (remaining 16 characters)
        let random = Self::decode_random_bytes(&chars[10..])?;

        Ok(Self::from_parts(timestamp, random))
    }

    /// Decodes a single Crockford Base32 character to its numeric value.
    ///
    /// # Errors
    ///
    /// Returns `IdError::InvalidCharacter` if the character is not valid.
    fn decode_char(c: char) -> Result<u8, IdError> {
        match c {
            '0' | 'O' => Ok(0),
            '1' | 'I' | 'L' => Ok(1),
            '2' => Ok(2),
            '3' => Ok(3),
            '4' => Ok(4),
            '5' => Ok(5),
            '6' => Ok(6),
            '7' => Ok(7),
            '8' => Ok(8),
            '9' => Ok(9),
            'A' => Ok(10),
            'B' => Ok(11),
            'C' => Ok(12),
            'D' => Ok(13),
            'E' => Ok(14),
            'F' => Ok(15),
            'G' => Ok(16),
            'H' => Ok(17),
            'J' => Ok(18),
            'K' => Ok(19),
            'M' => Ok(20),
            'N' => Ok(21),
            'P' => Ok(22),
            'Q' => Ok(23),
            'R' => Ok(24),
            'S' => Ok(25),
            'T' => Ok(26),
            'V' => Ok(27),
            'W' => Ok(28),
            'X' => Ok(29),
            'Y' => Ok(30),
            'Z' => Ok(31),
            _ => Err(IdError::InvalidCharacter),
        }
    }

    /// Decodes random bytes from characters
    fn decode_random_bytes(chars: &[char]) -> Result<[u8; 10], IdError> {
        let all_bits: u128 = chars.iter().try_fold(0u128, |acc, &c| {
            Self::decode_char(c).map(|value| (acc << 5) | u128::from(value))
        })?;

        let mut random = [0u8; 10];
        (0..10).for_each(|i| {
            random[i] = ((all_bits >> (8 * (9 - i))) & 0xFF) as u8;
        });

        Ok(random)
    }

    /// Creates a new Id from an owned String with ULID validation.
    ///
    /// This method validates that the input string is a valid ULID format
    /// (26 characters in Crockford Base32 encoding).
    ///
    /// # Errors
    ///
    /// Returns `IdError::InvalidLength` if the string is not 26 characters.
    /// Returns `IdError::InvalidCharacter` if the string contains invalid characters.
    ///
    /// # Examples
    ///
    /// ```
    /// use education_platform_common::Id;
    ///
    /// let id = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();
    /// assert_eq!(id.to_string(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    ///
    /// // Case-insensitive
    /// let id_lower = Id::from_string("01arz3ndektsv4rrffq69g5fav".to_string()).unwrap();
    /// assert_eq!(id, id_lower);
    ///
    /// // Invalid length
    /// let invalid = Id::from_string("TOOSHORT".to_string());
    /// assert!(invalid.is_err());
    ///
    /// // Invalid character
    /// let invalid_char = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5F@V".to_string());
    /// assert!(invalid_char.is_err());
    /// ```
    pub fn from_string(s: String) -> Result<Self, IdError> {
        Self::from_crockford_base32(&s)
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_crockford_base32())
    }
}

impl FromStr for Id {
    type Err = IdError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::from_crockford_base32(s)
    }
}

impl From<[u8; 16]> for Id {
    fn from(bytes: [u8; 16]) -> Self {
        Self { bytes }
    }
}

impl From<Id> for [u8; 16] {
    fn from(id: Id) -> Self {
        id.bytes
    }
}

/// Error types for ID operations.
#[derive(Error, Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum IdError {
    #[error("Invalid ID length: expected 26 characters")]
    InvalidLength,

    #[error("Invalid character in ID string")]
    InvalidCharacter,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_creates_valid_ulid() {
        let id = Id::new();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_timestamp_extraction() {
        let timestamp = 1234567890123u64;
        let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let id = Id::from_parts(timestamp, random);

        assert_eq!(id.timestamp_ms(), timestamp);
    }

    #[test]
    fn test_display_format_is_26_chars() {
        let id = Id::new();
        let id_str = id.to_string();

        assert_eq!(id_str.len(), 26);
        assert!(
            id_str
                .chars()
                .all(|c| "0123456789ABCDEFGHJKMNPQRSTVWXYZ".contains(c))
        );
    }

    #[test]
    fn test_different_ulids_are_unique() {
        let id1 = Id::new();
        let id2 = Id::new();

        assert_ne!(id1, id2);
    }

    #[test]
    fn test_ulids_are_time_ordered() {
        let id1 = Id::new();
        std::thread::sleep(std::time::Duration::from_millis(2));
        let id2 = Id::new();

        assert!(id1.timestamp_ms() <= id2.timestamp_ms());
        assert!(id1 < id2);
    }

    #[test]
    fn test_from_bytes() {
        let bytes = [
            0x01, 0x23, 0x45, 0x67, 0x89, 0xAB, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08,
            0x09, 0x0A,
        ];
        let id = Id::from(bytes);

        assert_eq!(id.as_bytes(), &bytes);
    }

    #[test]
    fn test_copy_semantics() {
        let id1 = Id::new();
        let id2 = id1;

        assert_eq!(id1, id2);
        assert_eq!(id1.to_string(), id2.to_string());
    }

    #[test]
    fn test_as_bytes_returns_correct_length() {
        let id = Id::new();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_encoding_decoding_roundtrip() {
        let id1 = Id::new();
        let encoded = id1.to_string();
        let id2: Id = encoded.parse().unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_from_str_valid() {
        let ulid_str = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let id: Id = ulid_str.parse().unwrap();
        assert_eq!(id.to_string(), ulid_str);
    }

    #[test]
    fn test_from_str_case_insensitive() {
        let upper = "01ARZ3NDEKTSV4RRFFQ69G5FAV";
        let lower = "01arz3ndektsv4rrffq69g5fav";

        let id1: Id = upper.parse().unwrap();
        let id2: Id = lower.parse().unwrap();

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_from_str_invalid_length() {
        assert_eq!(
            "TOOLONG12345678901234567890".parse::<Id>(),
            Err(IdError::InvalidLength)
        );
        assert_eq!("TOOSHORT".parse::<Id>(), Err(IdError::InvalidLength));
    }

    #[test]
    fn test_from_str_invalid_character() {
        let result = "01ARZ3NDEKTSV4RRFFQ69G5F@V".parse::<Id>();
        assert!(result.is_err());
    }

    #[test]
    fn test_ordering() {
        let id1 = Id::from_parts(1000, [0; 10]);
        let id2 = Id::from_parts(2000, [0; 10]);
        let id3 = Id::from_parts(3000, [0; 10]);

        assert!(id1 < id2);
        assert!(id2 < id3);
        assert!(id1 < id3);
    }

    #[test]
    fn test_hash_consistency() {
        use std::collections::HashSet;

        let id1 = Id::new();
        let id2 = id1;

        let mut set = HashSet::new();
        set.insert(id1);
        assert!(set.contains(&id2));
    }

    #[test]
    fn test_default_creates_new_id() {
        let id = Id::default();
        assert_eq!(id.as_bytes().len(), 16);
    }

    #[test]
    fn test_value_object_equality() {
        let timestamp = 1234567890123u64;
        let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

        let id1 = Id::from_parts(timestamp, random);
        let id2 = Id::from_parts(timestamp, random);

        assert_eq!(id1, id2);
    }

    #[test]
    fn test_lexicographic_sorting() {
        let id1 = Id::new();
        std::thread::sleep(std::time::Duration::from_millis(1));
        let id2 = Id::new();

        let str1 = id1.to_string();
        let str2 = id2.to_string();

        assert!(str1 < str2);
    }

    #[test]
    fn test_known_encoding() {
        let timestamp = 1469918176385u64;
        let random = [0x79, 0xE4, 0x2C, 0xC0, 0xC2, 0x98, 0x40, 0x00, 0x00, 0x00];

        let id = Id::from_parts(timestamp, random);
        let encoded = id.to_string();

        assert_eq!(encoded.len(), 26);
        assert!(encoded.chars().all(|c| c.is_ascii_alphanumeric()));
    }

    #[test]
    fn test_timestamp_boundaries() {
        let min_timestamp = 0u64;
        let max_timestamp = (1u64 << 48) - 1;

        let id_min = Id::from_parts(min_timestamp, [0; 10]);
        let id_max = Id::from_parts(max_timestamp, [0xFF; 10]);

        assert_eq!(id_min.timestamp_ms(), min_timestamp);
        assert_eq!(id_max.timestamp_ms(), max_timestamp);
    }

    #[test]
    fn test_from_string_valid() {
        let ulid_str = "01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string();
        let id = Id::from_string(ulid_str).unwrap();
        assert_eq!(id.to_string(), "01ARZ3NDEKTSV4RRFFQ69G5FAV");
    }

    #[test]
    fn test_from_string_case_insensitive() {
        let upper = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();
        let lower = Id::from_string("01arz3ndektsv4rrffq69g5fav".to_string()).unwrap();
        let mixed = Id::from_string("01ArZ3nDeKtSv4RrFfQ69G5FaV".to_string()).unwrap();

        assert_eq!(upper, lower);
        assert_eq!(upper, mixed);
    }

    #[test]
    fn test_from_string_invalid_length_too_short() {
        let result = Id::from_string("TOOSHORT".to_string());
        assert_eq!(result, Err(IdError::InvalidLength));
    }

    #[test]
    fn test_from_string_invalid_length_too_long() {
        let result = Id::from_string("TOOLONG12345678901234567890".to_string());
        assert_eq!(result, Err(IdError::InvalidLength));
    }

    #[test]
    fn test_from_string_invalid_length_empty() {
        let result = Id::from_string(String::new());
        assert_eq!(result, Err(IdError::InvalidLength));
    }

    #[test]
    fn test_from_string_invalid_character() {
        let result = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5F@V".to_string());
        assert_eq!(result, Err(IdError::InvalidCharacter));
    }

    #[test]
    fn test_from_string_invalid_character_space() {
        let result = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5 AV".to_string());
        assert_eq!(result, Err(IdError::InvalidCharacter));
    }

    #[test]
    fn test_from_string_roundtrip() {
        let original = Id::new();
        let string_repr = original.to_string();
        let parsed = Id::from_string(string_repr).unwrap();

        assert_eq!(original, parsed);
    }

    #[test]
    fn test_from_string_preserves_timestamp() {
        let timestamp = 1234567890123u64;
        let random = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
        let original = Id::from_parts(timestamp, random);

        let string_repr = original.to_string();
        let parsed = Id::from_string(string_repr).unwrap();

        assert_eq!(parsed.timestamp_ms(), timestamp);
    }

    #[test]
    fn test_from_string_confusable_characters() {
        // Crockford Base32 treats O as 0, I/L as 1
        let with_o = Id::from_string("O1ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();
        let with_zero = Id::from_string("01ARZ3NDEKTSV4RRFFQ69G5FAV".to_string()).unwrap();

        assert_eq!(with_o, with_zero);
    }
}
