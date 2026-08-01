//! ROT13: rotate ASCII letters by thirteen. Nothing else moves.

use alloc::string::String;

/// Rotate every ASCII letter of `s` by thirteen positions, wrapping within its own case.
///
/// Non-letters — digits, punctuation, whitespace, and every non-ASCII character — pass
/// through unchanged. ROT13 is its own inverse, so encoding and decoding are one function.
///
/// ```
/// use safe_decode::rot13;
/// assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
/// assert_eq!(rot13(&rot13("round trip")), "round trip");
/// ```
#[must_use]
pub fn rot13(s: &str) -> String {
    unimplemented!("RED: rot13")
}

#[cfg(test)]
mod tests {
    use super::rot13;

    #[test]
    fn rotates_both_cases() {
        assert_eq!(rot13("Hello, World!"), "Uryyb, Jbeyq!");
    }

    #[test]
    fn is_its_own_inverse() {
        let plain = "The quick brown fox jumps over the lazy dog.";
        assert_eq!(rot13(&rot13(plain)), plain);
    }

    #[test]
    fn wraps_at_the_alphabet_boundary() {
        // The halves either side of the wrap: A..M shift up, N..Z wrap down.
        assert_eq!(rot13("ABMNYZ"), "NOZALM");
        assert_eq!(rot13("abmnyz"), "nozalm");
    }

    #[test]
    fn leaves_non_letters_alone() {
        assert_eq!(rot13("0123456789"), "0123456789");
        assert_eq!(rot13(" \t\n{}[]|\\/@#"), " \t\n{}[]|\\/@#");
    }

    #[test]
    fn leaves_non_ascii_alone() {
        // Accented Latin, CJK and emoji are not ASCII letters and must not rotate.
        assert_eq!(rot13("café 日本語 🦀"), "pnsé 日本語 🦀");
    }

    #[test]
    fn empty_input_yields_empty_output() {
        assert_eq!(rot13(""), "");
    }

    #[test]
    fn decodes_a_real_userassist_value_name() {
        // UserAssist stores its value names ROT13'd; this is the shape they arrive in.
        assert_eq!(
            rot13(r"{P:\Jvaqbjf\flfgrz32\pzq.rkr"),
            r"{C:\Windows\system32\cmd.exe"
        );
    }
}
