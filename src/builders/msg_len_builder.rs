use crate::{builders::*, wire::WireVarInt, *};

/// Helper to determine the length values of a message (use [`MsgBuilder`]
/// to create).
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct MsgLenBuilder<'a> {
    pub(crate) buf: &'a mut MsgBuilder,
    pub(crate) cur_len: i32,
}

impl<'a> MsgLenBuilder<'a> {
    /// Adds the byte length of a protobuf `int32` field (VARINT encoded) to the
    /// current length.
    pub fn add_int32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int32_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `int64` field (VARINT encoded) to the
    /// current length.
    pub fn add_int64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int64_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `uint32` field (VARINT encoded) to the
    /// current length.
    pub fn add_uint32_field(&mut self, field_number: FieldNumber, value: u32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::uint32_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `uint64` field (VARINT encoded) to the
    /// current length.
    pub fn add_uint64_field(&mut self, field_number: FieldNumber, value: u64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::uint64_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `bool` field (VARINT encoded, always
    /// 1 byte) to the current length.
    pub fn add_bool_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 1;
        self
    }

    /// Adds the byte length of a protobuf `enum` field (VARINT encoded) to the
    /// current length.
    pub fn add_enum_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::int32_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `sint32` field (VARINT with ZigZag
    /// encoding) to the current length.
    pub fn add_sint32_field(&mut self, field_number: FieldNumber, value: i32) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::sint32_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `sint64` field (VARINT with ZigZag
    /// encoding) to the current length.
    pub fn add_sint64_field(&mut self, field_number: FieldNumber, value: i64) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + WireVarInt::sint64_byte_len(value);
        self
    }

    /// Adds the byte length of a protobuf `fixed32` field (4 bytes, little-endian)
    /// to the current length.
    pub fn add_fixed32_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    /// Adds the byte length of a protobuf `sfixed32` field (4 bytes, little-endian)
    /// to the current length.
    pub fn add_sfixed32_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    /// Adds the byte length of a protobuf `float` field (4 bytes, little-endian)
    /// to the current length.
    pub fn add_float_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 4;
        self
    }

    /// Adds the byte length of a protobuf `fixed64` field (8 bytes, little-endian)
    /// to the current length.
    pub fn add_fixed64_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    /// Adds the byte length of a protobuf `sfixed64` field (8 bytes, little-endian)
    /// to the current length.
    pub fn add_sfixed64_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    /// Adds the byte length of a protobuf `double` field (8 bytes, little-endian)
    /// to the current length.
    pub fn add_double_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.cur_len += WireVarInt::tag_byte_len(field_number) + 8;
        self
    }

    /// Adds the byte length of a protobuf `string` field (LEN encoded: tag +
    /// varint length + UTF-8 bytes) to the current length.
    pub fn add_string_field(&mut self, field_number: FieldNumber, value: &str) -> &mut Self {
        // TODO add length error
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(value.len() as i32)
            + (value.len() as i32);
        self
    }

    /// Adds the byte length of a protobuf `bytes` field (LEN encoded: tag +
    /// varint length + raw bytes) to the current length.
    pub fn add_bytes_field(&mut self, field_number: FieldNumber, value: &[u8]) -> &mut Self {
        // TODO add length error
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(value.len() as i32)
            + (value.len() as i32);
        self
    }

    /// Adds the byte length of a protobuf `string` field whose content is
    /// produced by the [`std::fmt::Display`] implementation of `value`.
    ///
    /// The length is determined by a byte-counting [`std::fmt::Write`] adapter
    /// that never allocates, using the same calculation as
    /// [`add_string_field`](Self::add_string_field) afterwards.
    pub fn add_display_str_field(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Display,
    ) -> Result<&mut Self, std::fmt::Error> {
        use std::fmt::Write;
        let mut counter = FmtByteCounter(0);
        write!(&mut counter, "{}", value)?;
        let len = counter.0;
        // same calculation as for str
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(len as i32)
            + (len as i32);
        Ok(self)
    }

    /// Adds the byte length of a protobuf `string` field whose content is
    /// produced by the [`std::fmt::Debug`] implementation of `value`.
    ///
    /// The length is determined by a byte-counting [`std::fmt::Write`] adapter
    /// that never allocates, using the same calculation as
    /// [`add_string_field`](Self::add_string_field) afterwards.
    pub fn add_debug_str_field(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Debug,
    ) -> Result<&mut Self, std::fmt::Error> {
        use std::fmt::Write;
        let mut counter = FmtByteCounter(0);
        write!(&mut counter, "{:?}", value)?;
        let len = counter.0;
        // same calculation as for str
        self.cur_len += WireVarInt::tag_byte_len(field_number)
            + WireVarInt::int32_byte_len(len as i32)
            + (len as i32);
        Ok(self)
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

    /// Begins length tracking for a nested sub-message field. Must be paired
    /// with a matching [`end_msg_field`](Self::end_msg_field) call using the
    /// same `field_number`.
    pub fn start_msg_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.start_len_area(field_number, LenStackType::Msg)
    }

    /// Ends length tracking for a nested sub-message field previously started
    /// with [`start_msg_field`](Self::start_msg_field). The accumulated byte
    /// length of the sub-message is stored so that the serialization phase can
    /// emit the correct varint length prefix.
    pub fn end_msg_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_len_area(field_number, LenStackType::Msg)
    }

    /// Begins length tracking for a packed repeated field and returns a
    /// [`MsgLenPackedScribe`] that can be used to add the packed elements.
    /// Must be paired with a matching [`end_packed_field`](Self::end_packed_field)
    /// call using the same `field_number`.
    pub fn start_packed_field<'b>(
        &'b mut self,
        field_number: FieldNumber,
    ) -> MsgLenPackedScribe<'a, 'b> {
        MsgLenPackedScribe {
            parent: self.start_len_area(field_number, LenStackType::Packed),
        }
    }

    /// Ends length tracking for a packed repeated field previously started
    /// with [`start_packed_field`](Self::start_packed_field). The accumulated
    /// byte length is stored so that the serialization phase can emit the
    /// correct varint length prefix.
    pub fn end_packed_field(&mut self, field_number: FieldNumber) -> &mut Self {
        self.end_len_area(field_number, LenStackType::Packed)
    }
}

impl<'a> MsgScribe for MsgLenBuilder<'a> {
    type Packed<'b>
        = MsgLenPackedScribe<'a, 'b>
    where
        Self: 'b;
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
    fn add_display_str(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Display,
    ) -> Result<&mut Self, std::fmt::Error> {
        self.add_display_str_field(field_number, value)
    }

    #[inline]
    fn add_debug_str(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Debug,
    ) -> Result<&mut Self, std::fmt::Error> {
        self.add_debug_str_field(field_number, value)
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

/// A [`std::fmt::Write`] implementation that only counts the number of
/// bytes written without storing them.
struct FmtByteCounter(usize);

impl std::fmt::Write for FmtByteCounter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0 += s.len();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::MsgBuilder;

    /// Helper to get `cur_len` after calling `add_display_str_field`.
    fn display_str_cur_len(field_number: FieldNumber, value: &dyn std::fmt::Display) -> i32 {
        let mut b = MsgBuilder::new();
        let mut len = b.start(None);
        len.add_display_str_field(field_number, value).unwrap();
        len.cur_len
    }

    /// Helper to get `cur_len` after calling `add_debug_str_field`.
    fn debug_str_cur_len(field_number: FieldNumber, value: &dyn std::fmt::Debug) -> i32 {
        let mut b = MsgBuilder::new();
        let mut len = b.start(None);
        len.add_debug_str_field(field_number, value).unwrap();
        len.cur_len
    }

    /// Helper to get `cur_len` after calling `add_string_field`.
    fn string_cur_len(field_number: FieldNumber, value: &str) -> i32 {
        let mut b = MsgBuilder::new();
        let mut len = b.start(None);
        len.add_string_field(field_number, value);
        len.cur_len
    }

    #[test]
    fn add_display_str_field() {
        // integer Display matches equivalent add_string_field
        {
            let f = FieldNumber(1);
            let value = 42u32;
            assert_eq!(
                display_str_cur_len(f, &value),
                string_cur_len(f, &format!("{}", value)),
            );
        }
        // empty Display output matches empty string
        {
            struct Empty;
            impl std::fmt::Display for Empty {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Ok(())
                }
            }
            let f = FieldNumber(1);
            assert_eq!(display_str_cur_len(f, &Empty), string_cur_len(f, ""));
        }
        // multi-byte UTF-8 content
        {
            let f = FieldNumber(1);
            let value = "Grüße 🌍";
            assert_eq!(display_str_cur_len(f, &value), string_cur_len(f, value),);
        }
        // large field number requiring a multi-byte varint tag
        {
            let f = FieldNumber(1000);
            let value = 9999i64;
            assert_eq!(
                display_str_cur_len(f, &value),
                string_cur_len(f, &format!("{}", value)),
            );
        }
        // failing Display impl propagates the error
        {
            struct Failing;
            impl std::fmt::Display for Failing {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Err(std::fmt::Error)
                }
            }
            let mut b = MsgBuilder::new();
            let mut len = b.start(None);
            assert!(len.add_display_str_field(FieldNumber(1), &Failing).is_err());
        }
    }

    #[test]
    fn add_debug_str_field() {
        // vector Debug matches equivalent add_string_field
        {
            let f = FieldNumber(2);
            let value = vec![1, 2, 3];
            assert_eq!(
                debug_str_cur_len(f, &value),
                string_cur_len(f, &format!("{:?}", value)),
            );
        }
        // Option Debug matches equivalent add_string_field
        {
            let f = FieldNumber(1);
            let value = Some("hello");
            assert_eq!(
                debug_str_cur_len(f, &value),
                string_cur_len(f, &format!("{:?}", value)),
            );
        }
        // failing Debug impl propagates the error
        {
            struct Failing;
            impl std::fmt::Debug for Failing {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Err(std::fmt::Error)
                }
            }
            let mut b = MsgBuilder::new();
            let mut len = b.start(None);
            assert!(len.add_debug_str_field(FieldNumber(1), &Failing).is_err());
        }
    }
}
