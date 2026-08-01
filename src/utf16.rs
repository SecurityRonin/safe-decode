//! UTF-16 decoding, with the NUL policy in the function name.
//!
//! Four policies exist in the wild and they disagree on the same bytes. Collapsing them
//! into one function with a flag would let a caller pick the wrong one silently, so each
//! is its own named function and there is no default.

use alloc::string::String;
use alloc::vec::Vec;

/// A decoded UTF-16 string together with what decoding it cost.
///
/// `text` is always well-formed UTF-8 — every unpaired surrogate half is replaced with
/// U+FFFD — so a caller may ignore the rest. The remaining fields say what the replacement
/// concealed, because a decode that lost information should be able to report it rather
/// than hand back a plausible-looking string.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub struct DecodedUtf16 {
    /// The decoded text. Unpaired surrogate halves appear as U+FFFD.
    pub text: String,
    /// How many unpaired surrogate halves were replaced with U+FFFD in `text`.
    pub unpaired_surrogates: usize,
    /// The input had an odd length, so a trailing byte could not form a code unit and was
    /// dropped. Reported for the input as a whole, whatever the NUL policy — an
    /// odd-length UTF-16 field is a structural anomaly worth surfacing even when the
    /// dropped byte fell past a terminator.
    pub dangling_byte: bool,
}

impl DecodedUtf16 {
    /// Whether anything was lost: an unpaired surrogate, a dangling byte, or both.
    #[must_use]
    pub fn is_lossy(&self) -> bool {
        unimplemented!("RED: DecodedUtf16::is_lossy")
    }
}

/// Decode the whole slice as UTF-16LE, keeping NUL code units as U+0000 characters.
///
/// No NUL is treated as a terminator. Use this when the field length is authoritative and
/// the bytes are exactly the string.
#[must_use]
pub fn decode_utf16le_keep_nuls(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16le_keep_nuls")
}

/// Decode UTF-16LE up to the first NUL code unit, which terminates the string.
///
/// Everything at and after the first NUL is discarded, including further strings. Use this
/// for a NUL-terminated field inside a larger buffer.
#[must_use]
pub fn decode_utf16le_until_nul(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16le_until_nul")
}

/// Decode the whole slice as UTF-16LE, then strip NUL code units from the end only.
///
/// Interior NULs are kept as U+0000. Use this for a fixed-width, NUL-padded field.
#[must_use]
pub fn decode_utf16le_trim_end_nuls(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16le_trim_end_nuls")
}

/// Split a UTF-16LE slice on NUL code units and decode every segment.
///
/// The split is structural and total: `n` NULs yield `n + 1` segments, empty ones
/// included, and an empty input yields one empty segment. A trailing NUL therefore
/// produces a trailing empty segment. Callers that want a terminator convention — such as
/// "stop at the first empty segment" — apply it themselves, where the knowledge of that
/// convention lives.
#[must_use]
pub fn split_utf16le_on_nul(bytes: &[u8]) -> Vec<DecodedUtf16> {
    unimplemented!("RED: split_utf16le_on_nul")
}

/// Decode the whole slice as UTF-16BE, keeping NUL code units as U+0000 characters.
///
/// The big-endian twin of [`decode_utf16le_keep_nuls`].
#[must_use]
pub fn decode_utf16be_keep_nuls(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16be_keep_nuls")
}

/// Decode UTF-16BE up to the first NUL code unit, which terminates the string.
///
/// The big-endian twin of [`decode_utf16le_until_nul`].
#[must_use]
pub fn decode_utf16be_until_nul(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16be_until_nul")
}

/// Decode the whole slice as UTF-16BE, then strip NUL code units from the end only.
///
/// The big-endian twin of [`decode_utf16le_trim_end_nuls`].
#[must_use]
pub fn decode_utf16be_trim_end_nuls(bytes: &[u8]) -> DecodedUtf16 {
    unimplemented!("RED: decode_utf16be_trim_end_nuls")
}

/// Split a UTF-16BE slice on NUL code units and decode every segment.
///
/// The big-endian twin of [`split_utf16le_on_nul`].
#[must_use]
pub fn split_utf16be_on_nul(bytes: &[u8]) -> Vec<DecodedUtf16> {
    unimplemented!("RED: split_utf16be_on_nul")
}

#[cfg(test)]
mod tests {
    use super::{
        decode_utf16be_keep_nuls, decode_utf16be_trim_end_nuls, decode_utf16be_until_nul,
        decode_utf16le_keep_nuls, decode_utf16le_trim_end_nuls, decode_utf16le_until_nul,
        split_utf16be_on_nul, split_utf16le_on_nul,
    };
    use alloc::string::String;
    use alloc::vec::Vec;

    /// UTF-16LE for 'A', NUL, 'B', NUL — an interior NUL *and* a trailing NUL, so all four
    /// policies must disagree. This one fixture is the whole reason the family exists.
    const FOUR_WAY_LE: &[u8] = &[0x41, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00, 0x00];
    /// The same four code units, big-endian.
    const FOUR_WAY_BE: &[u8] = &[0x00, 0x41, 0x00, 0x00, 0x00, 0x42, 0x00, 0x00];

    fn texts(parts: &[super::DecodedUtf16]) -> Vec<String> {
        parts.iter().map(|d| d.text.clone()).collect()
    }

    // ---- the four policies disagree on identical bytes -------------------------------

    #[test]
    fn keep_nuls_keeps_every_nul_as_u0000() {
        assert_eq!(decode_utf16le_keep_nuls(FOUR_WAY_LE).text, "A\0B\0");
    }

    #[test]
    fn until_nul_stops_dead_at_the_first_nul() {
        assert_eq!(decode_utf16le_until_nul(FOUR_WAY_LE).text, "A");
    }

    #[test]
    fn trim_end_nuls_keeps_interior_nuls_and_drops_only_trailing_ones() {
        assert_eq!(decode_utf16le_trim_end_nuls(FOUR_WAY_LE).text, "A\0B");
    }

    #[test]
    fn split_on_nul_yields_every_segment_including_the_trailing_empty() {
        assert_eq!(texts(&split_utf16le_on_nul(FOUR_WAY_LE)), ["A", "B", ""]);
    }

    #[test]
    fn the_four_policies_produce_four_different_answers() {
        let keep = decode_utf16le_keep_nuls(FOUR_WAY_LE).text;
        let until = decode_utf16le_until_nul(FOUR_WAY_LE).text;
        let trim = decode_utf16le_trim_end_nuls(FOUR_WAY_LE).text;
        let split = texts(&split_utf16le_on_nul(FOUR_WAY_LE)).join("|");
        let all = [keep.as_str(), until.as_str(), trim.as_str(), split.as_str()];
        for (i, a) in all.iter().enumerate() {
            for b in all.iter().skip(i + 1) {
                assert_ne!(a, b, "two policies collapsed onto the same answer");
            }
        }
    }

    // ---- the big-endian twins agree with their little-endian counterparts ------------

    #[test]
    fn big_endian_twins_match_the_little_endian_family() {
        assert_eq!(
            decode_utf16be_keep_nuls(FOUR_WAY_BE).text,
            decode_utf16le_keep_nuls(FOUR_WAY_LE).text
        );
        assert_eq!(
            decode_utf16be_until_nul(FOUR_WAY_BE).text,
            decode_utf16le_until_nul(FOUR_WAY_LE).text
        );
        assert_eq!(
            decode_utf16be_trim_end_nuls(FOUR_WAY_BE).text,
            decode_utf16le_trim_end_nuls(FOUR_WAY_LE).text
        );
        assert_eq!(
            texts(&split_utf16be_on_nul(FOUR_WAY_BE)),
            texts(&split_utf16le_on_nul(FOUR_WAY_LE))
        );
    }

    #[test]
    fn endianness_actually_changes_the_result() {
        // Reading LE bytes as BE must not silently produce the same string.
        assert_ne!(
            decode_utf16be_keep_nuls(&[0x41, 0x00]).text,
            decode_utf16le_keep_nuls(&[0x41, 0x00]).text
        );
    }

    // ---- lossiness is reported, never hidden -----------------------------------------

    #[test]
    fn well_formed_surrogate_pair_is_not_lossy() {
        // U+1F600 GRINNING FACE = D83D DE00.
        let d = decode_utf16le_keep_nuls(&[0x3D, 0xD8, 0x00, 0xDE]);
        assert_eq!(d.text, "\u{1F600}");
        assert_eq!(d.unpaired_surrogates, 0);
        assert!(!d.dangling_byte);
        assert!(!d.is_lossy());
    }

    #[test]
    fn lone_high_surrogate_is_replaced_and_counted() {
        let d = decode_utf16le_keep_nuls(&[0x00, 0xD8]);
        assert_eq!(d.text, "\u{FFFD}");
        assert_eq!(d.unpaired_surrogates, 1);
        assert!(!d.dangling_byte);
        assert!(d.is_lossy());
    }

    #[test]
    fn lone_low_surrogate_is_replaced_and_counted() {
        let d = decode_utf16le_keep_nuls(&[0x00, 0xDC]);
        assert_eq!(d.text, "\u{FFFD}");
        assert_eq!(d.unpaired_surrogates, 1);
        assert!(d.is_lossy());
    }

    #[test]
    fn unpaired_surrogate_before_a_valid_pair_counts_once() {
        // High, high, low — the first high is unpaired, the second pairs with the low.
        let d = decode_utf16le_keep_nuls(&[0x00, 0xD8, 0x3D, 0xD8, 0x00, 0xDE]);
        assert_eq!(d.text, "\u{FFFD}\u{1F600}");
        assert_eq!(d.unpaired_surrogates, 1);
        assert!(d.is_lossy());
    }

    #[test]
    fn odd_length_input_drops_the_trailing_byte_and_says_so() {
        let d = decode_utf16le_keep_nuls(&[0x41, 0x00, 0x42]);
        assert_eq!(d.text, "A");
        assert!(d.dangling_byte);
        assert_eq!(d.unpaired_surrogates, 0);
        assert!(d.is_lossy());
    }

    #[test]
    fn a_single_byte_decodes_to_nothing_but_reports_the_dangling_byte() {
        let d = decode_utf16be_keep_nuls(&[0x41]);
        assert_eq!(d.text, "");
        assert!(d.dangling_byte);
        assert!(d.is_lossy());
    }

    #[test]
    fn every_policy_reports_the_dangling_byte() {
        let odd: &[u8] = &[0x41, 0x00, 0x00, 0x00, 0x42];
        assert!(decode_utf16le_keep_nuls(odd).dangling_byte);
        assert!(decode_utf16le_until_nul(odd).dangling_byte);
        assert!(decode_utf16le_trim_end_nuls(odd).dangling_byte);
        assert!(decode_utf16be_keep_nuls(odd).dangling_byte);
        assert!(decode_utf16be_until_nul(odd).dangling_byte);
        assert!(decode_utf16be_trim_end_nuls(odd).dangling_byte);
    }

    #[test]
    fn split_reports_the_dangling_byte_on_the_segment_that_lost_it() {
        let parts = split_utf16le_on_nul(&[0x41, 0x00, 0x00, 0x00, 0x42]);
        assert_eq!(texts(&parts), ["A", ""]);
        assert!(!parts[0].dangling_byte);
        assert!(parts[1].dangling_byte);
        assert!(parts[1].is_lossy());
    }

    #[test]
    fn split_counts_unpaired_surrogates_per_segment() {
        // "A" NUL <lone high surrogate>
        let parts = split_utf16le_on_nul(&[0x41, 0x00, 0x00, 0x00, 0x00, 0xD8]);
        assert_eq!(texts(&parts), ["A", "\u{FFFD}"]);
        assert_eq!(parts[0].unpaired_surrogates, 0);
        assert!(!parts[0].is_lossy());
        assert_eq!(parts[1].unpaired_surrogates, 1);
        assert!(parts[1].is_lossy());
    }

    // ---- boundary inputs --------------------------------------------------------------

    #[test]
    fn empty_input_decodes_to_empty_and_is_not_lossy() {
        for d in [
            decode_utf16le_keep_nuls(&[]),
            decode_utf16le_until_nul(&[]),
            decode_utf16le_trim_end_nuls(&[]),
            decode_utf16be_keep_nuls(&[]),
            decode_utf16be_until_nul(&[]),
            decode_utf16be_trim_end_nuls(&[]),
        ] {
            assert_eq!(d.text, "");
            assert!(!d.is_lossy());
        }
    }

    #[test]
    fn empty_input_splits_into_one_empty_segment() {
        let parts = split_utf16le_on_nul(&[]);
        assert_eq!(texts(&parts), [""]);
        assert!(!parts[0].is_lossy());
    }

    #[test]
    fn all_nuls_are_handled_by_each_policy() {
        let nuls: &[u8] = &[0x00; 6];
        assert_eq!(decode_utf16le_keep_nuls(nuls).text, "\0\0\0");
        assert_eq!(decode_utf16le_until_nul(nuls).text, "");
        assert_eq!(decode_utf16le_trim_end_nuls(nuls).text, "");
        assert_eq!(texts(&split_utf16le_on_nul(nuls)), ["", "", "", ""]);
    }

    #[test]
    fn until_nul_returns_the_whole_string_when_no_nul_is_present() {
        let d = decode_utf16le_until_nul(&[0x41, 0x00, 0x42, 0x00]);
        assert_eq!(d.text, "AB");
        assert!(!d.is_lossy());
    }

    #[test]
    fn trim_end_nuls_leaves_a_string_without_padding_untouched() {
        assert_eq!(
            decode_utf16le_trim_end_nuls(&[0x41, 0x00, 0x42, 0x00]).text,
            "AB"
        );
    }

    #[test]
    fn decodes_a_realistic_nul_padded_path_field() {
        // "C:\ok" padded to a fixed 8-code-unit field with trailing NULs.
        let mut field: Vec<u8> = Vec::new();
        for u in "C:\\ok".encode_utf16() {
            field.extend_from_slice(&u.to_le_bytes());
        }
        field.resize(16, 0);
        assert_eq!(decode_utf16le_trim_end_nuls(&field).text, "C:\\ok");
        assert_eq!(decode_utf16le_until_nul(&field).text, "C:\\ok");
        assert_eq!(decode_utf16le_keep_nuls(&field).text, "C:\\ok\0\0\0");
    }
}
