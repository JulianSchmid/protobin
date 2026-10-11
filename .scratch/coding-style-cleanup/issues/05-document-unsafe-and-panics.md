# 05: Document `unsafe` blocks and panics, enforced by clippy

**What to build:** Every `unsafe` block explains why it is sound, and every method that can panic says so, as the coding style requires. Both rules become CI-enforced: the crate enables `clippy::undocumented_unsafe_blocks` and `clippy::missing_panics_doc`, and CI's `-D warnings` turns any new violation into a failure.

API misuse is the only allowed panic: unbalanced or mismatched `start_*`/`end_*` calls in the length phase, or a field number in the serialization phase that differs from the length phase. Callers usually reach these through the `MsgScribe` / `PackedScribe` trait methods, so the trait docs must say it too, not only the concrete builders.

**Blocked by:** None (can start immediately)

**Status:** done

- [x] Every `unsafe` block has a `// SAFETY:` comment naming the check that makes it sound
- [x] Every method that can panic has a `# Panics` doc section, including the `MsgScribe` / `PackedScribe` trait methods whose implementations panic on misuse
- [x] `clippy::undocumented_unsafe_blocks` and `clippy::missing_panics_doc` are enabled at crate level
- [x] All CI checks pass

## Comments

- `PackedScribe` has no `# Panics` sections, as none of its implementations panic on misuse. The `start_packed` / `end_packed` panics live on `MsgScribe`.
- `FieldNumber::get_next_unreserved` now panics on `FieldNumber::MAX_ALLOWED_U32` instead of returning the invalid `FieldNumber(0x20000000)`. That panic is reachable through input data, because `MsgDecoder` can produce the maximum field number, so the maintainer chose a `try_` variant: `try_get_next_unreserved` returns a `FieldNumberIntoError`, and `get_next_unreserved` is deprecated until 1.0.
- `MsgBuilder::start` skips a `num_len_values` hint that can't be reserved (`try_reserve`) instead of panicking.
- Still open: `MsgLenBuilder` and `MsgLenPackedScribe` add lengths with unchecked `i32` arithmetic and `len() as i32` casts. A message over `i32::MAX` bytes panics on overflow in debug builds and encodes wrong lengths in release builds. That breaks the "input data never panics or produces wrong bytes" hard rule, so it needs an error rather than a `# Panics` section. Tracked in ticket 14.
