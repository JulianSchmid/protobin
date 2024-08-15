use crate::{builders::*, *};

/// Helper to determine serialize a message after all
/// lengths have been determined.
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MsgSerBuilder<'a> {
    pub(crate) buf: &'a mut MsgBuilder,
    pub(crate) next_len_index: usize,
}

impl<'a> MsgSerBuilder<'a> {
    #[inline]
    fn add_varint_tag(&mut self, field_number: FieldNumber) {
        self.buf.encoder.add_var_uint32(field_number.0 << 3);
    }

    pub fn add_int32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_int32(value);
        self
    }

    pub fn add_int64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_int64(value);
        self
    }

    pub fn add_uint32_field(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_uint32(value);
        self
    }

    pub fn add_uint64_field(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_uint64(value);
        self
    }

    pub fn add_bool_field(&mut self, field_number: FieldNumber, value: bool) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_bool(value);
        self
    }

    pub fn add_enum_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf
            .encoder
            .add_var_uint32(u32::from_ne_bytes(value.to_ne_bytes()));
        self
    }

    pub fn add_sint32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_sint32(value);
        self
    }

    pub fn add_sint64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.add_varint_tag(field_number);
        self.buf.encoder.add_var_sint64(value);
        self
    }

    pub fn add_fixed32_field(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 5);
        self.buf.encoder.add_fixed32(value);
        self
    }

    pub fn add_sfixed32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 5);
        self.buf.encoder.add_sfixed32(value);
        self
    }

    pub fn add_float_field(&mut self, field_number: FieldNumber, value: f32) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 5);
        self.buf
            .encoder
            .add_fixed32(u32::from_ne_bytes(value.to_ne_bytes()));
        self
    }

    pub fn add_fixed64_field(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 1);
        self.buf.encoder.add_fixed64(value);
        self
    }

    pub fn add_sfixed64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 1);
        self.buf.encoder.add_sfixed64(value);
        self
    }

    pub fn add_double_field(&mut self, field_number: FieldNumber, value: f64) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 1);
        self.buf
            .encoder
            .add_fixed64(u64::from_ne_bytes(value.to_ne_bytes()));
        self
    }

    pub fn add_string_field(&mut self, field_number: FieldNumber, value: &str) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 2);
        self.buf.encoder.add_var_uint32(value.len() as u32);
        self.buf.encoder.buf.extend_from_slice(value.as_bytes());
        self
    }

    pub fn add_bytes_field(&mut self, field_number: FieldNumber, value: &[u8]) -> &mut Self {
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 2);
        self.buf.encoder.add_var_uint32(value.len() as u32);
        self.buf.encoder.buf.extend_from_slice(value);
        self
    }

    fn start_len_area(&mut self, field_number: FieldNumber) -> &mut Self {
        // get length value
        let len = self.buf.lens[self.next_len_index];
        self.next_len_index += 1;

        // check that the field number is matching
        assert_eq!(len.0, field_number, "Field number in serialisation does not match the one from the length pass (expected {}, actual {})", len.0.0, field_number.0);

        // write tag and length value
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 2);
        self.buf.encoder.add_var_int32(len.1);

        self
    }

    pub fn start_msg_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.start_len_area(field_number)
    }

    pub fn start_packed_field<'b>(
        &'b mut self,
        field_number: FieldNumber,
    ) -> MsgSerPackedScribe<'a, 'b> {
        MsgSerPackedScribe {
            parent: self.start_len_area(field_number),
        }
    }
}

impl<'a> MsgScribe for MsgSerBuilder<'a> {
    type Packed<'b> = MsgSerPackedScribe<'a, 'b> where Self: 'b;
    type End = &'a [u8];

    #[inline]
    fn add_int32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_int32_field(field_number, value)
    }

    #[inline]
    fn add_int64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.add_int64_field(field_number, value)
    }

    #[inline]
    fn add_uint32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.add_uint32_field(field_number, value)
    }

    #[inline]
    fn add_uint64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.add_uint64_field(field_number, value)
    }

    #[inline]
    fn add_bool(&mut self, field_number: FieldNumber, value: bool) -> &mut Self {
        self.add_bool_field(field_number, value)
    }

    #[inline]
    fn add_enum(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_enum_field(field_number, value)
    }

    #[inline]
    fn add_sint32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_sint32_field(field_number, value)
    }

    #[inline]
    fn add_sint64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.add_sint64_field(field_number, value)
    }

    #[inline]
    fn add_fixed32(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.add_fixed32_field(field_number, value)
    }

    #[inline]
    fn add_sfixed32(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.add_sfixed32_field(field_number, value)
    }

    #[inline]
    fn add_float(&mut self, field_number: FieldNumber, value: f32) -> &mut Self {
        self.add_float_field(field_number, value)
    }

    #[inline]
    fn add_fixed64(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.add_fixed64_field(field_number, value)
    }

    #[inline]
    fn add_sfixed64(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.add_sfixed64_field(field_number, value)
    }

    #[inline]
    fn add_double(&mut self, field_number: FieldNumber, value: f64) -> &mut Self {
        self.add_double_field(field_number, value)
    }

    #[inline]
    fn add_string(&mut self, field_number: FieldNumber, value: &str) -> &mut Self {
        self.add_string_field(field_number, value)
    }

    #[inline]
    fn add_bytes(&mut self, field_number: FieldNumber, value: &[u8]) -> &mut Self {
        self.add_bytes_field(field_number, value)
    }

    #[inline]
    fn start_msg(&mut self, field_number: FieldNumber) -> &mut Self {
        self.start_msg_field(field_number)
    }

    #[inline]
    fn end_msg(&mut self, _: FieldNumber) -> &mut Self {
        self
    }

    #[inline]
    fn start_packed<'b>(&'b mut self, field_number: FieldNumber) -> MsgSerPackedScribe<'a, 'b> {
        self.start_packed_field(field_number)
    }

    #[inline]
    fn end_packed(&mut self, _: FieldNumber) -> &mut Self {
        self
    }

    #[inline]
    fn end(self) -> Self::End {
        &self.buf.encoder.buf
    }
}
