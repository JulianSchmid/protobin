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

    pub fn add_display_str_field(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Display,
    ) -> Result<&mut Self, std::fmt::Error> {
        use std::fmt::Write;

        // count byte length of the formatted output
        let mut counter = FmtByteCounter(0);
        write!(&mut counter, "{}", value)?;
        let len = counter.0;

        // write tag + length
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 2);
        self.buf.encoder.add_var_uint32(len as u32);

        // write data directly into the buffer
        write!(FmtVecWriter(&mut self.buf.encoder.buf), "{}", value)?;

        Ok(self)
    }

    pub fn add_debug_str_field(
        &mut self,
        field_number: FieldNumber,
        value: &dyn std::fmt::Debug,
    ) -> Result<&mut Self, std::fmt::Error> {
        use std::fmt::Write;

        // count byte length of the formatted output
        let mut counter = FmtByteCounter(0);
        write!(&mut counter, "{:?}", value)?;
        let len = counter.0;

        // write tag + length
        self.buf.encoder.add_var_uint32((field_number.0 << 3) | 2);
        self.buf.encoder.add_var_uint32(len as u32);

        // write data directly into the buffer
        write!(FmtVecWriter(&mut self.buf.encoder.buf), "{:?}", value)?;

        Ok(self)
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

/// A [`std::fmt::Write`] implementation that only counts the number of
/// bytes written without storing them.
struct FmtByteCounter(usize);

impl std::fmt::Write for FmtByteCounter {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0 += s.len();
        Ok(())
    }
}

/// A [`std::fmt::Write`] implementation that writes UTF-8 bytes directly
/// into a [`Vec<u8>`].
struct FmtVecWriter<'a>(&'a mut Vec<u8>);

impl std::fmt::Write for FmtVecWriter<'_> {
    fn write_str(&mut self, s: &str) -> std::fmt::Result {
        self.0.extend_from_slice(s.as_bytes());
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::builders::MsgBuilder;
    use crate::decode::MsgDecoder;
    use crate::wire::WireValueRef;

    /// Helper: run a two-phase encode using `add_display_str_field` on both phases.
    fn encode_display(field_number: FieldNumber, value: &dyn std::fmt::Display) -> Vec<u8> {
        let mut builder = MsgBuilder::new();

        let mut len = builder.start(None);
        len.add_display_str_field(field_number, value).unwrap();
        let mut ser = len.end();
        ser.add_display_str_field(field_number, value).unwrap();
        ser.end().to_vec()
    }

    /// Helper: run a two-phase encode using `add_debug_str_field` on both phases.
    fn encode_debug(field_number: FieldNumber, value: &dyn std::fmt::Debug) -> Vec<u8> {
        let mut builder = MsgBuilder::new();

        let mut len = builder.start(None);
        len.add_debug_str_field(field_number, value).unwrap();
        let mut ser = len.end();
        ser.add_debug_str_field(field_number, value).unwrap();
        ser.end().to_vec()
    }

    /// Helper: run a two-phase encode using `add_string_field` on both phases.
    fn encode_string(field_number: FieldNumber, value: &str) -> Vec<u8> {
        let mut builder = MsgBuilder::new();

        let mut len = builder.start(None);
        len.add_string_field(field_number, value);
        let mut ser = len.end();
        ser.add_string_field(field_number, value);
        ser.end().to_vec()
    }

    /// Helper: decode the first (and only) field as a string.
    fn decode_single_string(bytes: &[u8], expected_field: FieldNumber) -> String {
        let mut decoder = MsgDecoder::new(bytes);
        let record = decoder.next().unwrap().unwrap();
        assert_eq!(record.field_number, expected_field);
        let s = match record.value {
            WireValueRef::Len(len_ref) => len_ref.try_as_string().unwrap().to_owned(),
            other => panic!("expected Len wire type, got {:?}", other),
        };
        assert!(decoder.next().is_none());
        s
    }

    #[test]
    fn add_display_str_field() {
        // integer Display produces the same bytes as add_string_field
        {
            let f = FieldNumber(1);
            let value = 42u32;
            assert_eq!(
                encode_display(f, &value),
                encode_string(f, &format!("{}", value)),
            );
        }
        // float Display decode round-trip
        {
            let f = FieldNumber(3);
            let value = 3.14f64;
            let bytes = encode_display(f, &value);
            assert_eq!(decode_single_string(&bytes, f), format!("{}", value));
        }
        // empty Display output matches empty string field
        {
            struct Empty;
            impl std::fmt::Display for Empty {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Ok(())
                }
            }
            let f = FieldNumber(1);
            assert_eq!(encode_display(f, &Empty), encode_string(f, ""));
        }
        // multi-byte UTF-8 content
        {
            let f = FieldNumber(1);
            let value = "Grüße 🌍";
            assert_eq!(encode_display(f, &value), encode_string(f, value));
        }
        // custom Display type encodes and decodes correctly
        {
            struct Point { x: i32, y: i32 }
            impl std::fmt::Display for Point {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    write!(f, "({}, {})", self.x, self.y)
                }
            }
            let f = FieldNumber(5);
            let value = Point { x: 10, y: -20 };
            let expected = format!("{}", value);
            assert_eq!(encode_display(f, &value), encode_string(f, &expected));
            assert_eq!(decode_single_string(&encode_display(f, &value), f), "(10, -20)");
        }
        // large field number requiring a multi-byte varint tag
        {
            let f = FieldNumber(1000);
            let value = "large field";
            assert_eq!(
                encode_display(f, &value),
                encode_string(f, &format!("{}", value)),
            );
        }
        // multiple display fields in one message
        {
            let f1 = FieldNumber(1);
            let f2 = FieldNumber(2);
            let v1 = 100u64;
            let v2 = "hello";

            let mut builder = MsgBuilder::new();
            let mut len = builder.start(None);
            len.add_display_str_field(f1, &v1).unwrap();
            len.add_display_str_field(f2, &v2).unwrap();
            let mut ser = len.end();
            ser.add_display_str_field(f1, &v1).unwrap();
            ser.add_display_str_field(f2, &v2).unwrap();
            let result = ser.end().to_vec();

            let mut decoder = MsgDecoder::new(&result);
            let rec1 = decoder.next().unwrap().unwrap();
            assert_eq!(rec1.field_number, f1);
            if let WireValueRef::Len(r) = rec1.value {
                assert_eq!(r.try_as_string().unwrap(), "100");
            } else {
                panic!("expected Len wire type for field 1");
            }
            let rec2 = decoder.next().unwrap().unwrap();
            assert_eq!(rec2.field_number, f2);
            if let WireValueRef::Len(r) = rec2.value {
                assert_eq!(r.try_as_string().unwrap(), "hello");
            } else {
                panic!("expected Len wire type for field 2");
            }
            assert!(decoder.next().is_none());
        }
        // failing Display impl propagates the error
        {
            struct Failing;
            impl std::fmt::Display for Failing {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Err(std::fmt::Error)
                }
            }
            let mut builder = MsgBuilder::new();
            let mut len = builder.start(None);
            assert!(len.add_display_str_field(FieldNumber(1), &Failing).is_err());
        }
    }

    #[test]
    fn add_debug_str_field() {
        // vector Debug produces the same bytes as add_string_field
        {
            let f = FieldNumber(2);
            let value = vec![1, 2, 3];
            assert_eq!(
                encode_debug(f, &value),
                encode_string(f, &format!("{:?}", value)),
            );
        }
        // Option Debug decode round-trip
        {
            let f = FieldNumber(4);
            let value = Some("hello");
            let bytes = encode_debug(f, &value);
            assert_eq!(decode_single_string(&bytes, f), format!("{:?}", value));
        }
        // multiple debug fields in one message
        {
            let f1 = FieldNumber(1);
            let f2 = FieldNumber(2);
            let v1 = 100u64;
            let v2 = vec!["a", "b"];

            let mut builder = MsgBuilder::new();
            let mut len = builder.start(None);
            len.add_debug_str_field(f1, &v1).unwrap();
            len.add_debug_str_field(f2, &v2).unwrap();
            let mut ser = len.end();
            ser.add_debug_str_field(f1, &v1).unwrap();
            ser.add_debug_str_field(f2, &v2).unwrap();
            let result = ser.end().to_vec();

            let mut decoder = MsgDecoder::new(&result);
            let rec1 = decoder.next().unwrap().unwrap();
            assert_eq!(rec1.field_number, f1);
            if let WireValueRef::Len(r) = rec1.value {
                assert_eq!(r.try_as_string().unwrap(), format!("{:?}", v1));
            } else {
                panic!("expected Len wire type for field 1");
            }
            let rec2 = decoder.next().unwrap().unwrap();
            assert_eq!(rec2.field_number, f2);
            if let WireValueRef::Len(r) = rec2.value {
                assert_eq!(r.try_as_string().unwrap(), format!("{:?}", v2));
            } else {
                panic!("expected Len wire type for field 2");
            }
            assert!(decoder.next().is_none());
        }
        // failing Debug impl propagates the error
        {
            struct Failing;
            impl std::fmt::Debug for Failing {
                fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    Err(std::fmt::Error)
                }
            }
            let mut builder = MsgBuilder::new();
            let mut len = builder.start(None);
            assert!(len.add_debug_str_field(FieldNumber(1), &Failing).is_err());
        }
    }
}
