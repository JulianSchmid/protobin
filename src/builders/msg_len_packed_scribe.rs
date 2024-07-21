use crate::{builders::*, wire::*};

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MsgLenPackedScribe<'a, 'b> {
    pub(crate) parent: &'b mut MsgLenBuilder<'a>,
}

impl<'a, 'b> PackedScribe for MsgLenPackedScribe<'a, 'b> {
    #[inline]
    fn add_int32(&mut self, value: i32) -> &mut Self {
        self.parent.cur_len += WireVarInt::int32_byte_len(value);
        self
    }

    #[inline]
    fn add_int64(&mut self, value: i64) -> &mut Self {
        self.parent.cur_len += WireVarInt::int64_byte_len(value);
        self
    }

    #[inline]
    fn add_uint32(&mut self, value: u32) -> &mut Self {
        self.parent.cur_len += WireVarInt::uint32_byte_len(value);
        self
    }

    #[inline]
    fn add_uint64(&mut self, value: u64) -> &mut Self {
        self.parent.cur_len += WireVarInt::uint64_byte_len(value);
        self
    }

    #[inline]
    fn add_bool(&mut self, _value: bool) -> &mut Self {
        self.parent.cur_len += 1;
        self
    }

    #[inline]
    fn add_enum(&mut self, value: i32) -> &mut Self {
        self.parent.cur_len += WireVarInt::int32_byte_len(value);
        self
    }

    #[inline]
    fn add_sint32(&mut self, value: i32) -> &mut Self {
        self.parent.cur_len += WireVarInt::sint32_byte_len(value);
        self
    }

    #[inline]
    fn add_sint64(&mut self, value: i64) -> &mut Self {
        self.parent.cur_len += WireVarInt::sint64_byte_len(value);
        self
    }

    #[inline]
    fn add_fixed32(&mut self, _value: u32) -> &mut Self {
        self.parent.cur_len += 4;
        self
    }

    #[inline]
    fn add_sfixed32(&mut self, _value: i32) -> &mut Self {
        self.parent.cur_len += 4;
        self
    }

    #[inline]
    fn add_float(&mut self, _value: f32) -> &mut Self {
        self.parent.cur_len += 4;
        self
    }

    #[inline]
    fn add_fixed64(&mut self, _value: u64) -> &mut Self {
        self.parent.cur_len += 8;
        self
    }

    #[inline]
    fn add_sfixed64(&mut self, _value: i64) -> &mut Self {
        self.parent.cur_len += 8;
        self
    }

    #[inline]
    fn add_double(&mut self, _value: f64) -> &mut Self {
        self.parent.cur_len += 8;
        self
    }
}
