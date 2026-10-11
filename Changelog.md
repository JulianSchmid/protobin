# Changelog

## 0.7.0

### Added

- `DecodeError` implements `Display`, `core::error::Error` and `Hash`.
- `WireValueIntoError` implements `Display` and `core::error::Error`.
- `FieldNumberIntoError` implements `Display` and `core::error::Error`, and `FieldNumberIntoError::invalid_value()` returns the value that was rejected.
- `WireValueRef::try_as_fixed32`, `try_as_sfixed32`, `try_as_float`, `try_as_fixed64`, `try_as_sfixed64` and `try_as_double`, following the `try_` prefix of the other fallible conversions.
- `# Panics` sections on the methods that panic on API misuse, including the `MsgScribe` methods: unbalanced or mismatched `start_*` / `end_*` calls in the length phase, and `start_*` calls in the serialization phase that don't match the length phase.

### Changed

- **Breaking:** `DecodeError::Len` is now `DecodeError::Len { required, remaining }` and reports how many bytes were needed and how many were left.
- **Breaking:** `DecodeError::UnexpectedContinuationBit` is now `DecodeError::UnexpectedContinuationBit { max_bytes }` and reports the maximum varint length (5 or 10 bytes).
- Raised MSRV from `1.65` to `1.81`, so the error types can implement `core::error::Error`.

### Deprecated

- `WireValueRef::as_fixed32`, `as_sfixed32`, `as_float`, `as_fixed64`, `as_sfixed64` and `as_double`. Use the `try_as_*` methods of the same type instead. The old names will be removed with 1.0.

### Fixed

- **Breaking:** `WireValueRef::try_as_bool` returns the new `WireValueIntoError::VarIntValueInvalidBool` for varints other than `0` or `1`, instead of `VarIntValueTooBigFor32Bit`.
- The fixed-size conversions on `WireValueRef` (`as_fixed32`, `as_double`, ..., now `try_as_fixed32`, `try_as_double`, ...) reported `WireType::VarInt` instead of `I32` / `I64` as the expected type in `WireValueIntoError::UnexpectedType`.
- `FieldNumber::get_next_unreserved` panics when called on the maximum field number, as its documentation said, instead of returning the invalid field number `0x20000000`.

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

- **Breaking:** `WireValueRef::try_as_bool` returns the new `WireValueIntoError::VarIntValueInvalidBool` for varints other than `0` or `1`, instead of `VarIntValueTooBigFor32Bit`.
- Corrected broken intra-doc links: `WireValue` -> `WireValueRef`, `MsgEncBuf` -> `MsgBuilder`, `ProtobufEncoder` -> `WireEncoder`.
