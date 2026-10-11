# Coding style

How code is written in protobin. Where existing code disagrees with a rule here, the rule wins: that code is awaiting cleanup, so copy the rule rather than the neighbour.

## Files & modules

- One public type per file, named after the type in snake case (`msg_len_builder.rs` → `MsgLenBuilder`). Error types get their own file too, in the module whose operations return them (`decode/decode_error.rs` → `DecodeError`).
- Import by globbing the modules you need: `use crate::{builders::*, wire::*, *};`.
- Use `core::` paths whenever the item exists in `core` (`core::fmt`, `core::ops::RangeInclusive`, `core::error::Error`); use `std::` only for items `core` lacks.

## Types

- Public types derive `Debug, Clone, Eq, PartialEq, Hash`, plus `Ord, PartialOrd` and `Default` where meaningful. Small value types (up to 128 bits, e.g. `FieldNumber`, `WireI32`) and borrowed views (`WireLenRef`, `WireValueRef`) also derive `Copy`. Types holding a `&mut` borrow derive what their fields allow.
- Prefer `const fn` wherever the body allows it.
- Range-restricted integer newtypes follow `FieldNumber`: a `pub(crate)` field, limit constants (`MAX_ALLOWED_U32`), `pub const fn try_from_<int>(value) -> Result<Self, _>`, `pub const unsafe fn from_<int>_unchecked(value)`, `pub const fn value(&self)`, `TryFrom<int>` and `From<Self> for <int>`.

## Naming

- Methods that can fail carry a `try_` prefix and return `Result` (`try_as_uint32`, `try_from_u32`). Infallible methods have no prefix (`WireI32::as_fixed32`).
- A renamed public method keeps its old name as a deprecated forwarder: `#[deprecated(since = "x.y.z", note = "Use `try_as_fixed32` instead")]`. Deprecated names are removed with 1.0.

## Errors & panics

- **Hard rule:** input data (values passed to the builders, bytes passed to the decoders) always leads to either correct output or a returned error, never a panic or silently wrong bytes. This covers casts: convert input-derived values with a checked conversion (`i32::try_from`) or a preceding range check, so an `as` cast only ever sees values that fit. Tests may `unwrap` freely.
- API misuse panics: unbalanced or mismatched `start_*`/`end_*` calls, or a different field number in the serialization pass than in the length pass. Every method that can panic documents it under `# Panics`.
- Every error type:
  - implements `Display` and `core::error::Error`,
  - derives `Clone, Debug, Eq, PartialEq, Hash`,
  - carries the offending values in field-shaped variants (`UnexpectedType { expected, actual }`, `UnknownWireType(u32)`),
  - writes `Display` messages that start with the context and quote values: `"Decode error: Unknown wire type '7'."`.

## `unsafe` & byte order

- Validate lengths once, then read with `get_unchecked` / `core::slice::from_raw_parts`.
- Every `unsafe` block gets a `// SAFETY:` comment naming the check that makes it sound, e.g. `// SAFETY: the length check above guarantees at least 4 bytes.`
- Every `unsafe fn` gets a `# Safety` doc section stating the complete precondition and asserts it with `debug_assert!` (see `FieldNumber::from_u32_unchecked`).
- Unchecked functions stay `pub(crate)` unless there is a reason to expose them (`from_u32_unchecked` is public so it works in `const` contexts).
- Protobuf fixed-size values (`fixed32`, `sfixed64`, `float`, `double`, ...) are little endian: convert with `to_le_bytes` / `from_le_bytes`, so the bytes are identical on every target.

## Performance & allocation

The crate aims to be as fast as possible. Encoding and decoding are the hot paths: keep them allocation-free and statically dispatched.

- Decoding never allocates. Decoders and `*Ref` types are zero-copy views that borrow from the input slice.
- Encoding allocates only in `MsgBuilder`'s reusable buffers. Clear them between messages instead of recreating them, so a reused `MsgBuilder` allocates nothing once its buffers have grown. Reserve capacity up front whenever the size is known.
- Public methods take borrowed input (`&str`, `&[u8]`, `&dyn Display`) rather than an owned `String` / `Vec`.
- Write straight into the output buffer, the way `add_display_str` formats through `core::fmt::Write` into the encoder. Library code never builds an intermediate `String`, `Vec`, `Box` or `collect`ed value.
- Use generics (static dispatch) on encode and decode paths. The exception is `core::fmt` values (`&dyn Display`, `&dyn Debug`): `core::fmt` dispatches dynamically internally, so generics would only add code bloat.
- Mark small functions on the encode and decode paths `#[inline]`.
- Back performance-motivated changes, especially new `unsafe` code, with numbers: run the benchmarks before and after, and put the comparison in the commit message or PR.

## Documentation

Write doc comments for users of the crate, short and sweet; implementation details and internal reasoning go in code comments.

- Every public item has a doc comment. Simple items (getters, constants, trivial constructors) get a one-line summary.
- Complicated items follow this order:
  1. a short description of what it does,
  2. a doc-test example,
  3. only the details a user needs: errors returned, edge cases that change the result, `# Panics`, `# Safety`.
- Link related items with intra-doc links, and point to the `try_` / unchecked alternatives where they exist.
- Types that implement part of the wire format link the matching section of the [protobuf encoding spec](https://protobuf.dev/programming-guides/encoding/).

## Tests

- Tests live at the bottom of the file in `#[cfg(test)] mod test { use super::*; ... }`.
- Prefer `proptest!` tests over hand-picked values.
- Coverage should be as high as possible: every branch, especially every error path, has a test.
- One test function per method, named after it (`fn try_as_uint32`), testing all of that method's branches. Separate cases go in `{ ... }` blocks inside it, each with a comment (`// normal case`, `// value too big for 32 bits`), and errors are checked with an exact `assert_eq!` on the expected error value. Round-trip tests of an encode/decode pair are named `from_and_as_<type>`. Other exceptions are fine where splitting genuinely reads better.
- Error types get `debug`, `clone_eq_hash`, `fmt` and `source` tests that pin the exact message strings.

## Commits

Commits are made for a human reviewer who reads a branch commit by commit; each one should be easy to review on its own.

- **One logical change per commit.** Keep refactors, renames, moves and formatting in separate commits from behaviour changes. Commit groundwork (a new error type, a wire helper, a refactor) first.
- **Build up in an order that reads well**, e.g. wire helpers and error types → `MsgScribe` / `PackedScribe` methods → `MsgLenBuilder` → `MsgSerBuilder` → packed scribes → README and Changelog.
- **Reviewability comes before a green build at every commit.** Intermediate commits may fail to compile or fail tests; the final state of the branch passes every CI check listed in `CLAUDE.md`.
- **Tests go in the same commit as the code they test.**
- **Messages:** a short imperative subject (`Add sint32 packed field support`). Add a body when the reason isn't obvious from the diff: what was wrong or missing and why this approach was chosen.
- Before opening a PR, clean up the history (squash fixups, reorder) so it tells this story, not the order the work happened in.
