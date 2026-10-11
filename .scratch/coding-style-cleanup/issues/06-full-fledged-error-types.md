# 06: Make error types full-fledged errors

**What to build:** `DecodeError`, `WireValueIntoError` and `FieldNumberIntoError` behave like proper Rust errors that users can print, chain and inspect, as the coding style requires:

- Each lives in its own file in the module whose operations return it. Public paths are unchanged.
- Each implements `Display` and `core::error::Error`, and derives `Clone, Debug, Eq, PartialEq, Hash`, keeping any existing extra derives such as `Ord`/`PartialOrd`.
- Every variant carries the values needed to explain it. In particular, `DecodeError`'s length variant reports how many bytes were required and how many were left, and users can read the invalid value from `FieldNumberIntoError`.
- `Display` messages start with the context and quote the values, e.g. `Decode error: Unknown wire type '7'.`

Changing `DecodeError`'s length variant breaks the API, which is acceptable in 0.7.0.

**Blocked by:** 01 (`core::error::Error` needs Rust 1.81)

**Status:** ready-for-agent

- [ ] Each error type is in its own file, and its public path is unchanged
- [ ] Each error type implements `Display` and `core::error::Error` and derives `Clone, Debug, Eq, PartialEq, Hash`
- [ ] Every variant carries its offending values, `DecodeError`'s length variant reports required vs. remaining bytes, and every decoder error site fills them in
- [ ] Users can read the invalid value from `FieldNumberIntoError`
- [ ] Each error type has `debug`, `clone_eq_hash`, `fmt` and `source` tests, and the `fmt` tests pin the exact message strings
- [ ] `Changelog.md` under `## 0.7.0` lists the new impls under "Added" and the breaking `DecodeError` change under "Changed"
- [ ] All CI checks pass
