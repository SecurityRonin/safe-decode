# 0002 — `MultiSz` and `MRUListEx` are registry conventions, not encodings

**Status:** Accepted

## Context

[ADR-0019 of the fleet constitution](https://github.com/SecurityRonin/ronin-issen/blob/main/docs/decisions/0019-foundation-algorithm-crates.md)
moves algorithms out of `forensicnomicon` — a knowledge crate — into primitive
crates, and lists `MultiSz` and `MruListEx` among the candidates for this one. It
also states the membership test each candidate must pass: **panic-free ·
allocating · format-agnostic · no domain knowledge.**

Both candidates are describable structurally, which is what makes the call
non-obvious:

- **`MultiSz`** (`REG_MULTI_SZ`) — UTF-16LE strings separated by NUL code units,
  the sequence terminated by an empty string, i.e. a double NUL.
- **`MRUListEx`** — little-endian `u32` values, the list terminated by
  `0xFFFFFFFF`.

## Decision

**`MultiSz` is split. The structural half ships; the convention does not.**

`split_utf16le_on_nul` (and its big-endian twin) performs the part that is a
property of the encoding: split a UTF-16 buffer on NUL code units and decode
every segment. It is total — `n` NULs yield `n + 1` segments, empty ones
included — so it describes the bytes and asserts nothing about them.

What it deliberately omits is everything specific to `REG_MULTI_SZ`: that the
sequence ends at a double NUL, that trailing empty segments are terminator
padding rather than data, and that empty strings in the middle should or should
not be dropped. Those are Windows registry facts. A caller composes them in one
line, in the crate where the fact already lives:

```rust
let values: Vec<String> = split_utf16le_on_nul(raw)
    .into_iter()
    .take_while(|s| !s.text.is_empty())   // REG_MULTI_SZ: the double NUL ends it
    .map(|s| s.text)
    .collect();
```

The dividing question is not "can this be described without naming Windows?"
Almost anything can. It is **"would this still be correct for a format I have
never seen that shares the structure?"** Splitting on NUL: yes. Stopping at the
double NUL and discarding the tail: only for formats that happen to agree with
the registry.

**`MRUListEx` is excluded entirely.**

The sentinel `0xFFFFFFFF` is not a property of a `u32` array — it is the registry
MRU convention. A different format terminates with a different sentinel, or a
count, or the field length. Generalising it to
`read_u32le_list_until(bytes, sentinel)` would produce a function whose only
content is three iterator combinators over `safe_read::le_u32`, with no bounds
risk to centralise and no behaviour to converge; there is no defect class it
prevents. The knowledge stays with `Decoder::MruListEx` in `forensicnomicon`,
which is where ADR-0019 says facts belong, and the read it performs already goes
through `safe-read`.

## Consequences

- `forensicnomicon` keeps the `MultiSz` and `MruListEx` *taxonomy entries*, as
  ADR-0019 intends; its `MultiSz` decoding delegates to `split_utf16le_on_nul`
  and applies the terminator rule itself. Its `MruListEx` loop stays where it is.
- The membership test stays decidable. Admitting `MRUListEx` — a named constant
  from one vendor's registry format — would have made "no domain knowledge"
  negotiable, and a primitive crate whose membership test is negotiable becomes
  the junk drawer ADR-0019 §3 exists to prevent.
