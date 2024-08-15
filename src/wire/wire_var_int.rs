use crate::*;

/// Variable sized integer Protobuf wire value used to
/// encode the Protobuf types `int32`, `int64`, `uint32`,
/// `uint64`, `sint32`, `sint64`, `bool` and `enum`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireVarInt(u64);

impl WireVarInt {
    #[inline]
    pub fn from_raw(raw_value: u64) -> WireVarInt {
        WireVarInt(raw_value)
    }

    /// Encode the given [`bool`] as Protobuf `bool`.
    #[inline]
    pub fn from_bool(value: bool) -> WireVarInt {
        if value {
            WireVarInt(1)
        } else {
            WireVarInt(0)
        }
    }

    /// Encode the given [`u32`] as Protobuf `uint32`.
    #[inline]
    pub fn from_uint32(value: u32) -> WireVarInt {
        WireVarInt(u64::from(value))
    }

    /// Encode the given [`i32`] as Protobuf `int32` (encodes using two’s complements).
    #[inline]
    pub fn from_int32(value: i32) -> WireVarInt {
        WireVarInt(u64::from(u32::from_ne_bytes(value.to_ne_bytes())))
    }

    /// Encode the given [`i32`] as Protobuf `sint32` (encodes using ZigZag).
    #[inline]
    pub fn from_sint32(value: i32) -> WireVarInt {
        let zigzag = (value << 1) ^ (value >> 31);
        WireVarInt(u64::from(u32::from_le_bytes(zigzag.to_ne_bytes())))
    }

    /// Encode the given [`u64`] as Protobuf `uint64`.
    #[inline]
    pub fn from_uint64(value: u64) -> WireVarInt {
        WireVarInt(value)
    }

    /// Encode the given [`i32`] as Protobuf `int64` (encodes using two’s complements).
    #[inline]
    pub fn from_int64(value: i64) -> WireVarInt {
        WireVarInt(u64::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Encode the given [`i64`] as Protobuf `sint64` (encodes using ZigZag).
    #[inline]
    pub fn from_sint64(value: i64) -> WireVarInt {
        let zigzag = (value << 1) ^ (value >> 63);
        WireVarInt(u64::from_ne_bytes(zigzag.to_ne_bytes()))
    }

    /// Returns the underlying stored encoded value (same as [`Self::as_uint64`]).
    #[inline]
    pub fn raw(&self) -> u64 {
        self.0
    }

    /// Try to interpret the varint as a Protobuf `bool`.
    ///
    /// If the varint contains any value other then `1` or `0`
    /// None is returned.
    pub fn try_as_bool(&self) -> Option<bool> {
        match self.0 {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }

    /// Try to interpret the varint as a Protobuf `uint32`.
    ///
    /// If the varint contains a value that is bigger then 32 bits
    /// `None` is returned instead.
    pub fn try_as_uint32(&self) -> Option<u32> {
        if self.0 > u64::from(u32::MAX) {
            None
        } else {
            Some(self.0 as u32)
        }
    }

    /// Try to interpret the varint as a Protobuf `int32` (encoded
    /// using two’s complements).
    ///
    /// If the varint contains a value that is bigger then 32 bits
    /// `None` is returned instead.
    pub fn try_as_int32(&self) -> Option<i32> {
        if self.0 > u64::from(u32::MAX) {
            None
        } else {
            Some(i32::from_ne_bytes((self.0 as u32).to_ne_bytes()))
        }
    }

    /// Try to interpret the varint as a Protobuf `sint32` (encoded
    /// using ZigZag).
    ///
    /// If the varint contains a value that is bigger then 32 bits
    /// `None` is returned instead.
    pub fn try_as_sint32(&self) -> Option<i32> {
        if self.0 > u64::from(u32::MAX) {
            None
        } else {
            let x = self.0 as u32;
            Some((x >> 1) as i32 ^ -((x & 1) as i32))
        }
    }

    /// Interpret the varint as a Protobuf `uint64`.
    pub fn as_uint64(&self) -> u64 {
        self.0
    }

    /// Interpret the varint as a Protobuf `int64` (encoded
    /// using two’s complements).
    pub fn as_int64(&self) -> i64 {
        i64::from_ne_bytes(self.0.to_ne_bytes())
    }

    /// Interpret the varint as a Protobuf `sint64` (encoded
    /// using ZigZag).
    pub fn as_sint64(&self) -> i64 {
        (self.0 >> 1) as i64 ^ -((self.0 & 1) as i64)
    }

    /// Returns the byte len of a encoded "tag" with the
    /// passed field number.
    ///
    /// Tag is the value written before a message field
    /// value is written.
    pub fn tag_byte_len(field_number: FieldNumber) -> i32 {
        if field_number.0 < (1 << (7 - 3)) {
            1
        } else if field_number.0 < (1 << ((7 * 2) - 3)) {
            2
        } else if field_number.0 < (1 << ((7 * 3) - 3)) {
            3
        } else if field_number.0 < (1 << ((7 * 4) - 3)) {
            4
        } else {
            5
        }
    }

    fn varint32_len(value: u32) -> i32 {
        if value < (1 << 7) {
            1
        } else if value < (1 << (7 * 2)) {
            2
        } else if value < (1 << (7 * 3)) {
            3
        } else if value < (1 << (7 * 4)) {
            4
        } else {
            5
        }
    }

    fn varint64_len(value: u64) -> i32 {
        if value < (1 << 7) {
            1
        } else if value < (1 << (7 * 2)) {
            2
        } else if value < (1 << (7 * 3)) {
            3
        } else if value < (1 << (7 * 4)) {
            4
        } else if value < (1 << (7 * 5)) {
            5
        } else if value < (1 << (7 * 6)) {
            6
        } else if value < (1 << (7 * 7)) {
            7
        } else if value < (1 << (7 * 8)) {
            8
        } else if value < (1 << (7 * 9)) {
            9
        } else {
            10
        }
    }

    /// Returns the encoded byte len of a "int32" encoded
    /// as varint (encoded using two’s complements).
    #[inline]
    pub fn int32_byte_len(value: i32) -> i32 {
        Self::varint32_len(u32::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Returns the encoded byte len of a "int64" encoded
    /// as varint (encoded using two’s complements).
    #[inline]
    pub fn int64_byte_len(value: i64) -> i32 {
        Self::varint64_len(u64::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Returns the encoded byte len of a "uint32" encoded
    /// as varint.
    #[inline]
    pub fn uint32_byte_len(value: u32) -> i32 {
        Self::varint32_len(value)
    }

    /// Returns the encoded byte len of a "uint64" encoded
    /// as varint.
    #[inline]
    pub fn uint64_byte_len(value: u64) -> i32 {
        Self::varint64_len(value)
    }

    /// Returns the encoded byte len of a "sint32" encoded
    /// as varint (encoded using zig zag).
    #[inline]
    pub fn sint32_byte_len(value: i32) -> i32 {
        let zigzag = (value << 1) ^ (value >> 31);
        Self::varint32_len(u32::from_le_bytes(zigzag.to_ne_bytes()))
    }

    /// Returns the encoded byte len of a "sint64" encoded
    /// as varint (encoded using zig zag).
    #[inline]
    pub fn sint64_byte_len(value: i64) -> i32 {
        let zigzag = (value << 1) ^ (value >> 63);
        Self::varint64_len(u64::from_ne_bytes(zigzag.to_ne_bytes()))
    }
}

#[cfg(test)]
mod tests {
    use crate::wire::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn from_and_as_bool(
            value in any::<bool>(),
            bad_value in 2..=u64::MAX,
        ) {
            // ok case
            {
                let v = WireVarInt::from_bool(value);
                prop_assert_eq!(Some(value), v.try_as_bool());
            }
            // err
            {
                let v = WireVarInt::from_raw(bad_value);
                prop_assert_eq!(None, v.try_as_bool());
            }
        }
    }

    proptest! {
        #[test]
        fn from_and_as_uint32(
            value in any::<u32>(),
            bad_value in (u32::MAX as u64 + 1)..=u64::MAX,
        ) {
            // ok case
            {
                let v = WireVarInt::from_uint32(value);
                prop_assert_eq!(Some(value), v.try_as_uint32());
            }
            // err
            {
                let v = WireVarInt::from_raw(bad_value);
                prop_assert_eq!(None, v.try_as_uint32());
            }
        }
    }

    #[test]
    fn from_sint32() {
        let tests = [
            (0i32, 0u64),
            (-1, 1),
            (1, 2),
            (-2, 3),
            (0x7fffffff, 0xfffffffe),
            (-0x80000000, 0xffffffff),
        ];
        for test in tests {
            let v = WireVarInt::from_sint32(test.0);
            assert_eq!(test.1, v.raw());
        }
    }

    proptest! {
        #[test]
        fn from_and_as_int32(
            value in any::<i32>(),
            bad_value in (u32::MAX as u64 + 1)..=u64::MAX,
        ) {
            // ok case
            {
                let v = WireVarInt::from_int32(value);
                prop_assert_eq!(Some(value), v.try_as_int32());
            }
            // err
            {
                let v = WireVarInt::from_raw(bad_value);
                prop_assert_eq!(None, v.try_as_int32());
            }
        }
    }

    proptest! {
        #[test]
        fn from_and_as_sint32(
            value in any::<i32>(),
            bad_value in (u32::MAX as u64 + 1)..=u64::MAX,
        ) {
            // ok case
            {
                let v = WireVarInt::from_sint32(value);
                prop_assert_eq!(Some(value), v.try_as_sint32());
            }
            // err
            {
                let v = WireVarInt::from_raw(bad_value);
                prop_assert_eq!(None, v.try_as_sint32());
            }
        }
    }

    proptest! {
        #[test]
        fn from_and_as_uint64(
            value in any::<u64>()
        ) {
            let v = WireVarInt::from_uint64(value);
            prop_assert_eq!(value, v.as_uint64());
        }
    }

    proptest! {
        #[test]
        fn from_and_as_int64(
            value in any::<i64>()
        ) {
            let v = WireVarInt::from_int64(value);
            prop_assert_eq!(value, v.as_int64());
        }
    }

    proptest! {
        #[test]
        fn from_and_as_sint64(
            value in any::<i64>()
        ) {
            let v = WireVarInt::from_sint64(value);
            prop_assert_eq!(value, v.as_sint64());
        }
    }

    #[test]
    fn from_sint64() {
        let tests = [
            (0i64, 0u64),
            (-1, 1),
            (1, 2),
            (-2, 3),
            (0x7fff_ffff, 0xffff_fffe),
            (-0x8000_0000, 0xffff_ffff),
            (0x7fff_ffff_ffff_ffff, 0xffff_ffff_ffff_fffe),
            (-0x8000_0000_0000_0000, 0xffff_ffff_ffff_ffff),
        ];
        for test in tests {
            let v = WireVarInt::from_sint64(test.0);
            assert_eq!(test.1, v.raw());
        }
    }
}
