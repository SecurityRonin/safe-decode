#![no_main]
//! `decode_utf16be_until_nul` must return a value for any byte slice and never panic.

use libfuzzer_sys::fuzz_target;
use safe_decode::decode_utf16be_until_nul;

fuzz_target!(|data: &[u8]| {
    let decoded = decode_utf16be_until_nul(data);
    // The dangling-byte flag reports an odd-length input, whatever the NUL policy.
    assert_eq!(decoded.dangling_byte, data.len() % 2 == 1);
    // `is_lossy` must agree with the two causes it summarises.
    assert_eq!(
        decoded.is_lossy(),
        decoded.dangling_byte || decoded.unpaired_surrogates > 0
    );
});
