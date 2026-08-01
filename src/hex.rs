//! Hex rendering of a byte slice.

use alloc::string::String;

/// Render `bytes` as lowercase hex, two characters per byte, no separator.
///
/// ```
/// use safe_decode::to_hex_lower;
/// assert_eq!(to_hex_lower(&[0x00, 0xde, 0xad]), "00dead");
/// ```
#[must_use]
pub fn to_hex_lower(bytes: &[u8]) -> String {
    unimplemented!("RED: to_hex_lower")
}

/// Render `bytes` as uppercase hex, two characters per byte, no separator.
///
/// ```
/// use safe_decode::to_hex_upper;
/// assert_eq!(to_hex_upper(&[0x00, 0xde, 0xad]), "00DEAD");
/// ```
#[must_use]
pub fn to_hex_upper(bytes: &[u8]) -> String {
    unimplemented!("RED: to_hex_upper")
}

#[cfg(test)]
mod tests {
    use super::{to_hex_lower, to_hex_upper};
    use alloc::vec::Vec;

    #[test]
    fn empty_input_yields_empty_output() {
        assert_eq!(to_hex_lower(&[]), "");
        assert_eq!(to_hex_upper(&[]), "");
    }

    #[test]
    fn pads_every_byte_to_two_digits() {
        // A leading zero nibble must survive; 0x0a is "0a", never "a".
        assert_eq!(to_hex_lower(&[0x00, 0x01, 0x0a, 0x0f]), "00010a0f");
    }

    #[test]
    fn renders_the_full_byte_range() {
        let all: Vec<u8> = (0u8..=255).collect();
        let lower = to_hex_lower(&all);
        assert_eq!(lower.len(), 512);
        assert!(lower.starts_with("000102"));
        assert!(lower.ends_with("fdfeff"));
        assert!(lower.chars().all(|c| c.is_ascii_hexdigit()));
        assert!(lower.chars().all(|c| !c.is_ascii_uppercase()));
    }

    #[test]
    fn upper_and_lower_differ_only_in_case() {
        let bytes: &[u8] = &[0xde, 0xad, 0xbe, 0xef, 0x01, 0x23];
        assert_eq!(to_hex_lower(bytes), "deadbeef0123");
        assert_eq!(to_hex_upper(bytes), "DEADBEEF0123");
        assert_eq!(
            to_hex_upper(bytes).to_ascii_lowercase(),
            to_hex_lower(bytes)
        );
    }

    #[test]
    fn output_length_is_always_twice_the_input() {
        for n in 0..64usize {
            let buf = alloc::vec![0xa5u8; n];
            assert_eq!(to_hex_lower(&buf).len(), n * 2);
            assert_eq!(to_hex_upper(&buf).len(), n * 2);
        }
    }
}
