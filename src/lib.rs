#![no_std]
#![forbid(unsafe_code)]
#![cfg_attr(
    test,
    allow(clippy::unwrap_used, clippy::expect_used, clippy::indexing_slicing)
)]

//! Panic-free, allocating byte→value transforms with no format knowledge.
//!
//! The companion to [`safe-read`](https://docs.rs/safe-read): that crate is `no_std` with
//! *no* allocator and reads fixed-width integers; this one needs `alloc` because its
//! outputs are `String`s and `Vec`s. That allocator boundary is the whole reason the two
//! are separate crates.
//!
//! Membership is decidable — every function here is **panic-free, allocating,
//! format-agnostic, and free of domain knowledge**. A transform that needs to know what
//! the bytes *mean* belongs in the crate that owns that knowledge, not here.
//!
//! ## The UTF-16 family names its NUL policy
//!
//! A fleet audit found fourteen hand-rolled UTF-16 decoders disagreeing about NUL in four
//! different ways. The disagreement was invisible because every one of them was called
//! `decode_utf16le`. Here each policy is a separately named function, so picking the wrong
//! one is a deliberate act rather than an accident:
//!
//! ```
//! use safe_decode::{
//!     decode_utf16le_keep_nuls, decode_utf16le_trim_end_nuls, decode_utf16le_until_nul,
//!     split_utf16le_on_nul,
//! };
//!
//! // UTF-16LE for 'A', NUL, 'B', NUL — the same bytes, four correct answers.
//! let bytes = b"A\0\0\0B\0\0\0";
//! assert_eq!(decode_utf16le_keep_nuls(bytes).text, "A\0B\0");
//! assert_eq!(decode_utf16le_until_nul(bytes).text, "A");
//! assert_eq!(decode_utf16le_trim_end_nuls(bytes).text, "A\0B");
//! let parts = split_utf16le_on_nul(bytes);
//! let texts: Vec<&str> = parts.iter().map(|d| d.text.as_str()).collect();
//! assert_eq!(texts, ["A", "B", ""]);
//! ```
//!
//! Each decode returns a [`DecodedUtf16`], which carries whether information was lost and
//! why — an unpaired surrogate half, or an odd trailing byte that could not form a code
//! unit. A caller that ignores it still gets well-formed text; a caller that cares can say
//! so in a report.
//!
//! ```
//! use safe_decode::decode_utf16le_keep_nuls;
//!
//! let lone_high_surrogate = decode_utf16le_keep_nuls(&[0x00, 0xD8]);
//! assert_eq!(lone_high_surrogate.text, "\u{FFFD}");
//! assert_eq!(lone_high_surrogate.unpaired_surrogates, 1);
//! assert!(lone_high_surrogate.is_lossy());
//! ```

extern crate alloc;

mod hex;
mod rot13;
mod utf16;

pub use hex::{to_hex_lower, to_hex_upper};
pub use rot13::rot13;
pub use utf16::{
    decode_utf16be_keep_nuls, decode_utf16be_trim_end_nuls, decode_utf16be_until_nul,
    decode_utf16le_keep_nuls, decode_utf16le_trim_end_nuls, decode_utf16le_until_nul,
    split_utf16be_on_nul, split_utf16le_on_nul, DecodedUtf16,
};
