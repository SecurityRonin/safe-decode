# safe-decode

[![Crates.io](https://img.shields.io/crates/v/safe-decode.svg)](https://crates.io/crates/safe-decode)
[![Docs.rs](https://docs.rs/safe-decode/badge.svg)](https://docs.rs/safe-decode)
[![Rust 1.75+](https://img.shields.io/badge/rust-1.75%2B-blue.svg)](rust-toolchain.toml)
[![License: Apache-2.0](https://img.shields.io/badge/License-Apache--2.0-blue.svg)](LICENSE)
[![Sponsor](https://img.shields.io/badge/Sponsor-h4x0r-ea4aaa.svg)](https://github.com/sponsors/h4x0r)

[![CI](https://github.com/SecurityRonin/safe-decode/actions/workflows/ci.yml/badge.svg)](https://github.com/SecurityRonin/safe-decode/actions/workflows/ci.yml)
[![unsafe forbidden](https://img.shields.io/badge/unsafe-forbidden-success.svg)](https://doc.rust-lang.org/reference/behavior-considered-undefined.html)
[![Security advisories](https://img.shields.io/badge/advisories-clean-success.svg)](https://rustsec.org/)

**Byte-to-text transforms that name what they do to NUL.** Fourteen hand-rolled UTF-16 decoders across one codebase, four different NUL policies, and every one of them called `decode_utf16le`. This crate ends that: the policy is in the function name, so choosing the wrong one is a decision rather than an accident.

```rust
use safe_decode::{
    decode_utf16le_keep_nuls, decode_utf16le_trim_end_nuls, decode_utf16le_until_nul,
    split_utf16le_on_nul,
};

// UTF-16LE 'A', NUL, 'B', NUL — the same bytes, four correct answers.
let bytes = b"A\0\0\0B\0\0\0";
assert_eq!(decode_utf16le_keep_nuls(bytes).text, "A\0B\0");      // NUL is a character
assert_eq!(decode_utf16le_until_nul(bytes).text, "A");           // NUL terminates
assert_eq!(decode_utf16le_trim_end_nuls(bytes).text, "A\0B");    // NUL is padding
let parts = split_utf16le_on_nul(bytes);                         // NUL separates
assert_eq!(parts.iter().map(|d| d.text.as_str()).collect::<Vec<_>>(), ["A", "B", ""]);
```

## Install

```toml
[dependencies]
safe-decode = "0.1"
```

## A decode that lost something says so

Every decode returns a `DecodedUtf16`, not a bare `String`. `text` is always well-formed UTF-8, so a caller may ignore the rest — but the rest is what a forensic report needs, because "the field decoded" and "the field decoded faithfully" are different claims.

```rust
use safe_decode::decode_utf16le_keep_nuls;

let d = decode_utf16le_keep_nuls(&[0x41, 0x00, 0x00, 0xD8, 0x42]);
assert_eq!(d.text, "A\u{FFFD}");
assert_eq!(d.unpaired_surrogates, 1); // a lone high surrogate, replaced
assert!(d.dangling_byte);             // odd length: a byte could not form a code unit
assert!(d.is_lossy());
```

## What's here

| Function | Behaviour |
|---|---|
| `rot13` | Rotate ASCII letters by thirteen; everything else, including non-ASCII, unchanged. |
| `decode_utf16le_keep_nuls` · `decode_utf16be_keep_nuls` | Decode the whole slice; NUL code units become U+0000 characters. |
| `decode_utf16le_until_nul` · `decode_utf16be_until_nul` | Stop at the first NUL; discard it and everything after. |
| `decode_utf16le_trim_end_nuls` · `decode_utf16be_trim_end_nuls` | Decode all, then strip trailing NULs only; interior NULs kept. |
| `split_utf16le_on_nul` · `split_utf16be_on_nul` | Split on NUL and decode every segment, empty ones included. |
| `to_hex_lower` · `to_hex_upper` | Two hex digits per byte, no separator. |

## Trust, but verify

- **Fuzzed.** A `cargo-fuzz` target per public function drives arbitrary byte slices; a panic is a build failure, and `cargo fuzz check` runs in CI.
- **Panic-free by lint and by construction.** `unwrap_used`, `expect_used` and `indexing_slicing` are `deny`, and `unsafe_code` is `forbid`. Code units come from `chunks_exact` through a fallible array conversion rather than an index; truncation goes through `Vec::truncate` rather than a range slice; the hex digit is masked to a nibble. There is no construct in the crate that can panic on any input.
- **100% line and function coverage**, gated in CI.
- **Zero dependencies**, `#![no_std]` plus `alloc`, MSRV 1.75 verified in CI — nothing here raises a consumer's floor.

## Scope

A function belongs in this crate only if it is **panic-free, allocating, format-agnostic, and free of domain knowledge**. `alloc` is the line between this crate and [`safe-read`](https://crates.io/crates/safe-read), its `no_std`-without-allocator sibling for fixed-width integer reads.

Deliberately absent: anything that needs to know what the bytes *mean*. `REG_MULTI_SZ`'s double-NUL terminator and `MRUListEx`'s `0xFFFFFFFF` sentinel are Windows registry conventions, not properties of an encoding, so they live with the crate that owns that knowledge. `split_utf16le_on_nul` gives that caller the structural half; the convention stays where the fact is. See [`docs/decisions/`](docs/decisions/).

---

[Privacy Policy](https://securityronin.github.io/safe-decode/privacy/) · [Terms of Service](https://securityronin.github.io/safe-decode/terms/) · © 2026 Security Ronin Ltd
