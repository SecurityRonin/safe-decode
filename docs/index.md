# safe-decode

Panic-free, allocating byte-to-value transforms with no format knowledge. The
companion to [`safe-read`](https://crates.io/crates/safe-read): that crate is
`no_std` with no allocator and reads fixed-width integers; this one needs `alloc`
because its outputs are `String`s and `Vec`s. That allocator boundary is the whole
reason the two are separate crates.

## The UTF-16 family names its NUL policy

Four NUL policies exist in the wild and they disagree on the same bytes. Each is
its own named function, and there is no default:

```rust
use safe_decode::{
    decode_utf16le_keep_nuls, decode_utf16le_trim_end_nuls, decode_utf16le_until_nul,
    split_utf16le_on_nul,
};

let bytes = b"A\0\0\0B\0\0\0"; // UTF-16LE 'A', NUL, 'B', NUL
assert_eq!(decode_utf16le_keep_nuls(bytes).text, "A\0B\0");
assert_eq!(decode_utf16le_until_nul(bytes).text, "A");
assert_eq!(decode_utf16le_trim_end_nuls(bytes).text, "A\0B");
let parts = split_utf16le_on_nul(bytes);
assert_eq!(parts.iter().map(|d| d.text.as_str()).collect::<Vec<_>>(), ["A", "B", ""]);
```

Each returns a `DecodedUtf16` carrying whether information was lost and why — an
unpaired surrogate half, or an odd trailing byte that could not form a code unit.

## Also here

- `rot13` — rotate ASCII letters by thirteen, everything else unchanged.
- `to_hex_lower` / `to_hex_upper` — two hex digits per byte, no separator.

## Membership

A function belongs here only if it is panic-free, allocating, format-agnostic and
free of domain knowledge. See [Purpose & Scope](PRD.md) and the
[decision records](https://github.com/SecurityRonin/safe-decode/tree/main/docs/decisions).

---

[Privacy Policy](privacy.md) · [Terms of Service](terms.md) · © 2026 Security Ronin Ltd
