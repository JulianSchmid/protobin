use crate::{builders::*, wire::WireVarInt, *};

/// Helper to determine the length values of a message (use [`MsgEnc`]
/// to create).
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MsgLenBuilder<'a> {
    pub(crate) buf: &'a mut MsgBuilder,
    pub(crate) cur_len: i32,
}

impl<'a> MsgLenBuilder<'a> {
    pub fn add_int32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int32_byte_len(value);
        self
    }

    pub fn add_int64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int64_byte_len(value);
        self
    }

    pub fn add_uint32_field(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::uint32_byte_len(value);
        self
    }

    pub fn add_uint64_field(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::uint64_byte_len(value);
        self
    }

    pub fn add_bool_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 1;
        self
    }

    pub fn add_enum_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int32_byte_len(value);
        self
    }

    pub fn add_sint32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::sint32_byte_len(value);
        self
    }

    pub fn add_sint64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::sint64_byte_len(value);
        self
    }

    pub fn add_fixed32_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    pub fn add_sfixed32_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    pub fn add_float_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    pub fn add_fixed64_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    pub fn add_sfixed64_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    pub fn add_double_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    pub fn add_string_field(&mut self, field_number: FieldNumber, value: &str) -> &mut Self {
        // TODO add length error
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(value.len() as i32)
            + (value.len() as i32);
        self
    }

    pub fn add_bytes_field(&mut self, field_number: FieldNumber, value: &[u8]) -> &mut Self {
        // TODO add length error
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(value.len() as i32)
            + (value.len() as i32);
        self
    }

    fn start_len_area(&mut self, field_number: FieldNumber, t: LenStackType) -> &mut Self {
        // add tag (length skipped until after message is done)
        self.cur_len += WireVarInt::tag_byte_len(field_number);

        // save length in stack
        if let Some(e) = self.buf.len_stack.last_mut() {
            e.len = self.cur_len;
        }

        // add length stack entry
        self.cur_len = 0;
        self.buf.lens.push((field_number, 0));
        self.buf.len_stack.push(LenStackEntry {
            len: 0,
            t,
            len_index: self.buf.lens.len() - 1,
            field_number,
        });

        self
    }

    fn end_len_area(&mut self, field_number: FieldNumber, t: LenStackType) -> &mut Self {
        // finalize length
        let Some(ended) = self.buf.len_stack.pop() else {
            panic!("'end_msg_field' or 'end_packed_field' called but no corresponding 'start_msg_field' or 'start_packed_field' left to be closed.");
        };
        assert_eq!(
            (ended.field_number, ended.t),
            (field_number, t),
            "Unexpected end call, expected {:?} {:?} but got {field_number:?} {t:?}",
            ended.field_number,
            ended.t
        );
        self.buf.lens[ended.len_index].1 = self.cur_len;

        // restore cur_len
        if let Some(next) = self.buf.len_stack.last() {
            self.cur_len += next.len + WireVarInt::int32_byte_len(self.cur_len);
        } // else just keep the cur_len (makes no difference, no stack left)

        self
    }

    pub fn start_msg_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.start_len_area(field_number, LenStackType::Msg)
    }

    pub fn end_msg_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_len_area(field_number, LenStackType::Msg)
    }

    pub fn start_packed_field<'b>(
        &'b mut self,
        field_number: FieldNumber,
    ) -> MsgLenPackedScribe<'a, 'b> {
        MsgLenPackedScribe {
            parent: self.start_len_area(field_number, LenStackType::Packed),
        }
    }

    pub fn end_packed_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_len_area(field_number, LenStackType::Packed)
    }
}

impl<'a> MsgScribe for MsgLenBuilder<'a> {
    type Packed<'b> = MsgLenPackedScribe<'a, 'b> where Self: 'b;
    type End = MsgSerBuilder<'a>;

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
    fn add_bool(&mut self, field_number: FieldNumber, _value: bool) -> &mut Self {
        self.add_bool_field(field_number)
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
    fn add_fixed32(&mut self, field_number: FieldNumber, _value: u32) -> &mut Self {
        self.add_fixed32_field(field_number)
    }

    #[inline]
    fn add_sfixed32(&mut self, field_number: FieldNumber, _value: i32) -> &mut Self {
        self.add_sfixed32_field(field_number)
    }

    #[inline]
    fn add_float(&mut self, field_number: FieldNumber, _value: f32) -> &mut Self {
        self.add_float_field(field_number)
    }

    #[inline]
    fn add_fixed64(&mut self, field_number: FieldNumber, _value: u64) -> &mut Self {
        self.add_fixed64_field(field_number)
    }

    #[inline]
    fn add_sfixed64(&mut self, field_number: FieldNumber, _value: i64) -> &mut Self {
        self.add_sfixed64_field(field_number)
    }

    #[inline]
    fn add_double(&mut self, field_number: FieldNumber, _value: f64) -> &mut Self {
        self.add_double_field(field_number)
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
    fn end_msg(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_msg_field(field_number)
    }

    #[inline]
    fn start_packed<'b>(&'b mut self, field_number: FieldNumber) -> MsgLenPackedScribe<'a, 'b> {
        self.start_packed_field(field_number)
    }

    #[inline]
    fn end_packed(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_packed_field(field_number)
    }

    #[inline]
    fn end(self) -> Self::End {
        assert!(
            self.buf.len_stack.is_empty(),
            "Overall end called before all submessages or packed data were ended"
        );
        MsgSerBuilder {
            buf: self.buf,
            next_len_index: 0,
        }
    }
}
