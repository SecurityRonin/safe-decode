#![no_main]
//! `to_hex_upper` must render any byte slice as uppercase hex and never panic.

use libfuzzer_sys::fuzz_target;
use safe_decode::to_hex_upper;

fuzz_target!(|data: &[u8]| {
    let hex = to_hex_upper(data);
    // Exactly two digits per byte, all of them hex, none of them the wrong case.
    assert_eq!(hex.len(), data.len() * 2);
    assert!(hex.chars().all(|c| c.is_ascii_hexdigit()));
    assert!(hex.chars().all(|c| !c.is_ascii_lowercase()));
});
