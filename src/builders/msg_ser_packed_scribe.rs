use crate::builders::*;

#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MsgSerPackedScribe<'a, 'b> {
    pub(crate) parent: &'b mut MsgSerBuilder<'a>,
}

impl<'a, 'b> PackedScribe for MsgSerPackedScribe<'a, 'b> {
    #[inline]
    fn add_int32(&mut self, value: i32) -> &mut Self {
        self.parent.buf.encoder.add_var_int32(value);
        self
    }

    #[inline]
    fn add_int64(&mut self, value: i64) -> &mut Self {
        self.parent.buf.encoder.add_var_int64(value);
        self
    }

    #[inline]
    fn add_uint32(&mut self, value: u32) -> &mut Self {
        self.parent.buf.encoder.add_var_uint32(value);
        self
    }

    #[inline]
    fn add_uint64(&mut self, value: u64) -> &mut Self {
        self.parent.buf.encoder.add_var_uint64(value);
        self
    }

    #[inline]
    fn add_bool(&mut self, value: bool) -> &mut Self {
        self.parent.buf.encoder.add_bool(value);
        self
    }

    #[inline]
    fn add_enum(&mut self, value: i32) -> &mut Self {
        self.parent.buf.encoder.add_var_int32(value);
        self
    }

    #[inline]
    fn add_sint32(&mut self, value: i32) -> &mut Self {
        self.parent.buf.encoder.add_var_sint32(value);
        self
    }

    #[inline]
    fn add_sint64(&mut self, value: i64) -> &mut Self {
        self.parent.buf.encoder.add_var_sint64(value);
        self
    }

    #[inline]
    fn add_fixed32(&mut self, value: u32) -> &mut Self {
        self.parent.buf.encoder.add_fixed32(value);
        self
    }

    #[inline]
    fn add_sfixed32(&mut self, value: i32) -> &mut Self {
        self.parent.buf.encoder.add_sfixed32(value);
        self
    }

    #[inline]
    fn add_float(&mut self, value: f32) -> &mut Self {
        self.parent.buf.encoder.add_float(value);
        self
    }

    #[inline]
    fn add_fixed64(&mut self, value: u64) -> &mut Self {
        self.parent.buf.encoder.add_fixed64(value);
        self
    }

    #[inline]
    fn add_sfixed64(&mut self, value: i64) -> &mut Self {
        self.parent.buf.encoder.add_sfixed64(value);
        self
    }

    #[inline]
    fn add_double(&mut self, value: f64) -> &mut Self {
        self.parent.buf.encoder.add_double(value);
        self
    }
}
