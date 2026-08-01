# safe-decode — Purpose & Scope

**What it is.** `safe-decode` is the fleet's single implementation of the
byte-to-value transforms that need an allocator: ROT13, the UTF-16 decoder family,
and hex rendering. Nothing here knows what the bytes mean. Every function is
panic-free on any input, and every UTF-16 decode reports whether it lost
information rather than handing back a plausible-looking string.

**Who uses it.** Any fleet crate that turns bytes into text. It exists because a
DRY audit found ROT13 implemented four times and UTF-16 decoded fourteen times,
the latter with four mutually incompatible NUL policies all sharing the name
`decode_utf16le` — a defect class that is invisible at every call site.

**In scope.** A transform qualifies only if it is **panic-free, allocating,
format-agnostic, and free of domain knowledge** — all four. `#![no_std]` plus
`alloc`, zero dependencies, `#![forbid(unsafe_code)]`, and a low CI-verified MSRV
(1.75) so no consumer's floor is raised.

**The boundary with `safe-read`.** `safe-read` is `no_std` with no allocator; its
outputs are integers. This crate's outputs are `String`s and `Vec`s, so it needs
`alloc`. That is not a stylistic split — it is why the UTF-16 decoders could not
simply be added to `safe-read`, and it is the membership line between the two.

**Out of scope (someone else's job).**

- **Temporal conversion.** FILETIME, WebKit and Unix epochs, and every sentinel
  policy around them, belong to `timeglyph-core`. One home for time.
- **Format conventions dressed as encodings.** `REG_MULTI_SZ`'s double-NUL
  terminator and `MRUListEx`'s `0xFFFFFFFF` sentinel are Windows registry facts,
  not properties of UTF-16 or of a `u32` array. This crate supplies the structural
  half (`split_utf16le_on_nul`); the convention stays with the crate that owns the
  fact.
- **Anything needing a format catalog**, and anything with its own crate (ESE,
  protobuf, LEB128 varints as defined by a specific format).
- **Semantic validation.** A `safe-decode` call guarantees "no panic, and an
  honest account of what was lost". It does not guarantee the bytes were a
  well-formed instance of anything.

**Deliberately no default.** There is no plain `decode_utf16le`, and no policy
enum or flag parameter. A caller must name the NUL policy it wants. The audit
showed that a shared, unqualified name is precisely what let four incompatible
behaviours coexist unnoticed for years.
