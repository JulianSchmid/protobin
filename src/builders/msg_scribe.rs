use crate::FieldNumber;

use super::PackedScribe;

/// Trait abstracting the two phases of protobuf message encoding.
///
/// Because protobuf uses length-prefixed sub-messages and packed fields, the
/// encoding is split into two phases:
///
/// 1. **Length phase** ([`MsgLenBuilder`](super::MsgLenBuilder)) -- calculates
///    the byte lengths of all length-delimited regions.
/// 2. **Serialization phase** ([`MsgSerBuilder`](super::MsgSerBuilder)) --
///    writes the actual encoded bytes using the pre-calculated lengths.
///
/// Both phases implement `MsgScribe`, so a single serialization function
/// can drive both phases by being generic over `S: MsgScribe`.
pub trait MsgScribe {
    /// The packed-field scribe type returned by [`start_packed`](Self::start_packed).
    type Packed<'a>: PackedScribe
    where
        Self: 'a;

    /// The value returned by [`end`](Self::end) when the message is complete.
    ///
    /// For the length phase this is the [`MsgSerBuilder`](super::MsgSerBuilder)
    /// (kicking off phase 2); for the serialization phase it is the encoded
    /// byte slice `&[u8]`.
    type End;

    /// Adds a protobuf `int32` field (VARINT, two's complement).
    fn add_int32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    /// Adds a protobuf `int64` field (VARINT, two's complement).
    fn add_int64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;
    /// Adds a protobuf `uint32` field (VARINT).
    fn add_uint32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self;
    /// Adds a protobuf `uint64` field (VARINT).
    fn add_uint64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self;
    /// Adds a protobuf `bool` field (VARINT, single byte).
    fn add_bool(&mut self, field_number: FieldNumber, value: bool) -> &mut Self;
    /// Adds a protobuf `enum` field (VARINT).
    fn add_enum(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    /// Adds a protobuf `sint32` field (VARINT, ZigZag encoded).
    fn add_sint32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    /// Adds a protobuf `sint64` field (VARINT, ZigZag encoded).
    fn add_sint64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;

    /// Adds a protobuf `fixed32` field (4 bytes, little-endian).
    fn add_fixed32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self;
    /// Adds a protobuf `sfixed32` field (4 bytes, little-endian).
    fn add_sfixed32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    /// Adds a protobuf `float` field (4 bytes, little-endian IEEE 754).
    fn add_float(&mut self, field_number: FieldNumber, value: f32) -> &mut Self;

    /// Adds a protobuf `fixed64` field (8 bytes, little-endian).
    fn add_fixed64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self;
    /// Adds a protobuf `sfixed64` field (8 bytes, little-endian).
    fn add_sfixed64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;
    /// Adds a protobuf `double` field (8 bytes, little-endian IEEE 754).
    fn add_double(&mut self, field_number: FieldNumber, value: f64) -> &mut Self;

    /// Adds a protobuf `string` field (LEN wire type: tag + varint length +
    /// UTF-8 bytes).
    fn add_string(&mut self, field_number: FieldNumber, value: &str) -> &mut Self;
    /// Adds a protobuf `bytes` field (LEN wire type: tag + varint length +
    /// raw bytes).
    fn add_bytes(&mut self, field_number: FieldNumber, value: &[u8]) -> &mut Self;

    /// Adds a protobuf `string` field by formatting the given value using its
    /// [`std::fmt::Display`] implementation directly into the serialization
    /// buffer, avoiding intermediate [`String`] allocations.
    fn add_display_str(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Display,
    ) -> Result<&mut Self, std::fmt::Error>;

    /// Adds a protobuf `string` field by formatting the given value using its
    /// [`std::fmt::Debug`] implementation directly into the serialization
    /// buffer, avoiding intermediate [`String`] allocations.
    fn add_debug_str(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Debug,
    ) -> Result<&mut Self, std::fmt::Error>;

    /// Begins a nested sub-message field. All fields added after this call
    /// (until the matching [`end_msg`](Self::end_msg)) belong to the
    /// sub-message. Must be paired with `end_msg` using the same
    /// `field_number`.
    fn start_msg(&mut self, field_number: FieldNumber) -> &mut Self;
    /// Ends a nested sub-message field previously started with
    /// [`start_msg`](Self::start_msg).
    fn end_msg(&mut self, field_number: FieldNumber) -> &mut Self;

    /// Begins a packed repeated field and returns a [`PackedScribe`] that can
    /// be used to add the packed elements. Must be paired with
    /// [`end_packed`](Self::end_packed) using the same `field_number`.
    fn start_packed<'a>(&'a mut self, field_number: FieldNumber) -> Self::Packed<'a>;
    /// Ends a packed repeated field previously started with
    /// [`start_packed`](Self::start_packed).
    fn end_packed(&mut self, field_number: FieldNumber) -> &mut Self;

    /// Finishes the current encoding phase and returns the phase result.
    ///
    /// For [`MsgLenBuilder`](super::MsgLenBuilder) this returns a
    /// [`MsgSerBuilder`](super::MsgSerBuilder) to begin the serialization
    /// phase. For [`MsgSerBuilder`](super::MsgSerBuilder) this returns the
    /// final encoded byte slice.
    fn end(self) -> Self::End;

    // TODO add packed method with slice passing
}
