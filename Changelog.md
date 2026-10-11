# Changelog

## 0.7.0

### Added

- `DecodeError` implements `Display`, `core::error::Error` and `Hash`.

### Changed

- **Breaking:** `DecodeError::Len` is now `DecodeError::Len { required, remaining }` and reports how many bytes were needed and how many were left.
- Raised MSRV from `1.65` to `1.81`, so the error types can implement `core::error::Error`.

## 0.6.0

### Added

- `add_display_str` / `add_display_str_field` on `MsgScribe`, `MsgLenBuilder`, and `MsgSerBuilder` -- serialize a `Display` value directly as a protobuf `string` field without allocating an intermediate `String`.
- `add_debug_str` / `add_debug_str_field` on `MsgScribe`, `MsgLenBuilder`, and `MsgSerBuilder` -- serialize a `Debug` value directly as a protobuf `string` field without allocating an intermediate `String`.
- Comprehensive documentation: doc comments on all public methods in `MsgScribe`, `MsgLenBuilder`, and `MsgSerBuilder`; trait-level doc on `MsgScribe` explaining the two-phase encoding design.
- `#![doc = include_str!("../README.md")]` in `lib.rs` so the README is rendered on docs.rs.
- README rewritten with badges, motivation section, encoding explanation, usage examples (simple message, nested messages, decoding, inspecting unknown data), and a supported-types table.
- Set MSRV to `1.65` (`rust-version` field in `Cargo.toml`).

### Changed

- Introduced explicit lifetime annotations on `MsgBuilder::start`, `MsgDecoder::new`, `WireDecoder::new`, and `MsgScribe::start_packed` (previously elided).

### Fixed

- Corrected broken intra-doc links: `WireValue` -> `WireValueRef`, `MsgEncBuf` -> `MsgBuilder`, `ProtobufEncoder` -> `WireEncoder`.
