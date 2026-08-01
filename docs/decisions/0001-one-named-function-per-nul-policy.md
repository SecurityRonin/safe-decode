# 0001 — One named function per NUL policy, and no default

**Status:** Accepted

## Context

A fleet DRY audit found at least fourteen hand-rolled UTF-16 decoders. They
implemented four incompatible NUL policies:

| Policy | Seen in | On `'A' NUL 'B' NUL` |
|---|---|---|
| Decode everything; NUL is a character | `lnk-forensic`, `leveldb-forensic`, `chromium-storage` (BE) | `"A\0B\0"` |
| Stop at the first NUL | `winreg-core` | `"A"` |
| Strip trailing NULs only | `dpapi-forensic` | `"A\0B"` |
| Split on NUL into many strings | `winreg-core`, `forensicnomicon` | `["A", "B", ""]` |

Almost all of them were named `decode_utf16le`. The signature was identical in
every case — `fn(&[u8]) -> String` — so nothing at a call site distinguished
them, nothing failed when a consumer reached for the wrong one, and the
disagreement was invisible in review.

Two shapes were considered for consolidation:

1. **One function plus a policy argument** — `decode_utf16le(bytes, NulPolicy::Trim)`.
2. **One named function per policy** — no default, no flag.

## Decision

**Each NUL policy is its own function, named for the policy, and there is no
plain `decode_utf16le`.**

```
decode_utf16le_keep_nuls        decode_utf16be_keep_nuls
decode_utf16le_until_nul        decode_utf16be_until_nul
decode_utf16le_trim_end_nuls    decode_utf16be_trim_end_nuls
split_utf16le_on_nul            split_utf16be_on_nul
```

A policy argument was rejected because it reintroduces the defect one layer
down: an argument has a value that can be wrong, a call that reads plausibly can
still be wrong, and something must be the ergonomic choice. Naming makes the
policy part of the identifier the reviewer already reads. The absence of a
default is deliberate — there is no name a caller can reach for without having
decided.

**Endianness is a separate axis and the matrix is closed.** Endianness affects
only the byte-pair-to-code-unit step and is orthogonal to the NUL policy, so
every policy exists in both. Shipping only the five combinations with a current
consumer would leave holes, and a hole in a primitive is filled by hand at the
call site — which is how fourteen decoders came to exist.

**Decoding is total, and lossiness is returned rather than logged or dropped.**
The return type is `DecodedUtf16 { text, unpaired_surrogates, dangling_byte }`,
not `String` and not `Result`. `text` is always well-formed, so a caller may
ignore the rest; a caller writing a forensic report can distinguish "the field
decoded" from "the field decoded faithfully". `leveldb-forensic` already tracked
a `lossy` flag and was the only one of the fourteen to do so — that is the model
here, split into its two distinct causes so a report can name which occurred.

## Consequences

- Fourteen implementations converge on eight functions whose behaviour is
  legible from the name.
- A migrating consumer must state which policy it was relying on. Where the
  answer is not obvious from the surrounding code this is a genuine finding, not
  friction — an unexamined NUL policy over evidence bytes is a defect waiting to
  be attributed.
- `dangling_byte` reports an odd-length *input*, uniformly, whatever the policy.
  Under `until_nul` this can flag a byte that fell past the terminator and was
  never part of the string. Over-reporting a structural anomaly in evidence is
  the safe direction; hiding it is not.
