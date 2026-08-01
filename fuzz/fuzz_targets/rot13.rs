#![no_main]
//! `rot13` must round-trip any string and never panic.

use libfuzzer_sys::fuzz_target;
use safe_decode::rot13;

fuzz_target!(|data: &[u8]| {
    let text = String::from_utf8_lossy(data);
    let once = rot13(&text);
    // ROT13 is an involution and touches only ASCII letters, so the character
    // count is preserved and applying it twice restores the input exactly.
    assert_eq!(once.chars().count(), text.chars().count());
    assert_eq!(rot13(&once), text);
});
