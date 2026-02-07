use crate::FieldNumber;

use super::PackedScribe;

pub trait MsgScribe {
    type Packed<'a>: PackedScribe
    where
        Self: 'a;
    type End;

    fn add_int32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    fn add_int64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;
    fn add_uint32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self;
    fn add_uint64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self;
    fn add_bool(&mut self, field_number: FieldNumber, value: bool) -> &mut Self;
    fn add_enum(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    fn add_sint32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    fn add_sint64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;

    fn add_fixed32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self;
    fn add_sfixed32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self;
    fn add_float(&mut self, field_number: FieldNumber, value: f32) -> &mut Self;

    fn add_fixed64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self;
    fn add_sfixed64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self;
    fn add_double(&mut self, field_number: FieldNumber, value: f64) -> &mut Self;

    fn add_string(&mut self, field_number: FieldNumber, value: &str) -> &mut Self;
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

    fn start_msg(&mut self, field_number: FieldNumber) -> &mut Self;
    fn end_msg(&mut self, field_number: FieldNumber) -> &mut Self;

    fn start_packed<'a>(&'a mut self, field_number: FieldNumber) -> Self::Packed<'a>;
    fn end_packed(&mut self, field_number: FieldNumber) -> &mut Self;

    fn end(self) -> Self::End;

    // TODO add packed method with slice passing
}
