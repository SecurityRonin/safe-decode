#![no_main]
//! `split_utf16be_on_nul` must split any byte slice totally and never panic.

use libfuzzer_sys::fuzz_target;
use safe_decode::split_utf16be_on_nul;

fuzz_target!(|data: &[u8]| {
    let segments = split_utf16be_on_nul(data);
    // The split is total: `n` NUL code units yield `n + 1` segments, so there is
    // always at least one and no input can lose a segment.
    assert_eq!(segments.len(), nul_units(data) + 1);
    for segment in &segments {
        assert_eq!(
            segment.is_lossy(),
            segment.dangling_byte || segment.unpaired_surrogates > 0
        );
    }
});

/// Count NUL code units independently of the crate. A NUL code unit is two zero
/// bytes whichever way round they are read, so this holds for both endiannesses.
fn nul_units(data: &[u8]) -> usize {
    data.chunks_exact(2).filter(|c| c == &[0, 0]).count()
}
