# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

CI (`.github/workflows/rust.yml`) runs all of the following; a PR must pass every one:

```sh
cargo build
cargo test                                   # unit tests, proptests and README doctests
cargo fmt -- --check
cargo clippy -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps
cargo +1.65 build                            # MSRV check (build only; dev-deps need a newer toolchain)
```

Run a subset of tests with a name filter, e.g. `cargo test msg_ser_builder` or `cargo test --doc`.

Examples: `cargo run --example gen_test_msg -- out.proto out.bin` (also `gen_complex_msg`) writes a `.proto` schema plus an encoded message for checking against external protobuf tooling. `cargo run --example print_wire_types -- out.bin` dumps the records of any protobuf binary.

## Constraints

- **MSRV is Rust 1.65** (`rust-version` in `Cargo.toml`). `MsgScribe::Packed<'b>` is a generic associated type and the builders use `let ... else`, both of which only just made it into 1.65. Don't use newer language or std features.
- **No runtime dependencies.** `proptest` is the only dev-dependency.
- **`README.md` is the crate-level doc** (`#![doc = include_str!("../README.md")]` in `src/lib.rs`). Its Rust code blocks run as doctests, with `#`-prefixed lines hidden, and its intra-doc links (e.g. `` [`MsgScribe`](builders::MsgScribe) ``) must resolve or the `cargo doc -D warnings` job fails.
- Commit new files under `proptest-regressions/`, since proptest replays those seeds on every run.
- Record user-visible changes in `Changelog.md` under the current version heading.

## Architecture

The public API has three modules plus `FieldNumber` at the crate root. Each submodule file is private and re-exported flat with `pub use x::*` from its `mod.rs`, so the public paths are `protobin::builders::MsgBuilder`, `protobin::wire::WireEncoder` and so on.

- `wire`: wire-format primitives. These are varint/zig-zag handling (`WireVarInt`), fixed 32- and 64-bit values, `WireEncoder` (an append-only byte buffer), `WireLenCalc` (computes encoded sizes without writing), `WireDecoder` and the borrowed `WireValueRef`/`WireLenRef` views with their `try_as_*`/`as_*` conversions.
- `decode`: `MsgDecoder` is a zero-copy iterator over `MsgRecordRef { field_number, value }`. Nested messages are decoded by calling `WireLenRef::as_sub_msg()` to get a sub-decoder.
- `builders`: two-phase encoding, described below.

### Two-phase encoding (`src/builders/`)

Every LEN field (sub-message or packed repeated field) is prefixed with its byte length as a varint, and that prefix's size depends on the length itself. Instead of writing placeholders and shifting bytes afterwards, encoding runs the same user-written function twice:

1. `MsgBuilder::start()` clears its reusable buffers and returns a `MsgLenBuilder`. This phase writes no bytes. It adds up sizes in `cur_len` and, on each `start_msg`/`start_packed`, pushes an entry onto `MsgBuilder.len_stack` and reserves a slot in `MsgBuilder.lens`. On the matching `end_*` it pops the entry, stores the finished length in the slot and adds the child's size (including its length prefix) back into the parent.
2. `MsgLenBuilder::end()` returns a `MsgSerBuilder`. This phase writes into `MsgBuilder.encoder` and reads `lens` **sequentially** through `next_len_index` whenever a LEN area starts. Its `end_msg`/`end_packed` do nothing, and `end()` returns `&[u8]`.

Both phase types implement `MsgScribe` (`PackedScribe` for the packed-element scribes), so users write one generic `fn ser<S: MsgScribe>(..., s: S)`. The resulting invariant is that **both passes must call `start_*`/`end_*` in exactly the same order with the same field numbers**. Mismatches panic: an unbalanced or mismatched `end_*` panics in the length phase, and a field-number mismatch panics in the serialization phase.

To add a new field type, you usually touch all of these:

- the `MsgScribe` trait (`msg_scribe.rs`);
- the inherent `add_*_field` method and the trait impl on `MsgLenBuilder` (`msg_len_builder.rs`);
- the same pair on `MsgSerBuilder` (`msg_ser_builder.rs`);
- for packable scalars, `PackedScribe` and its two implementations (`msg_len_packed_scribe.rs`, `msg_ser_packed_scribe.rs`);
- the matching size and encoding helpers in `wire/`.

Length-phase methods for fixed-size types omit the value (e.g. `MsgLenBuilder::add_fixed32_field(field_number)`). Lengths are tracked as `i32`.
