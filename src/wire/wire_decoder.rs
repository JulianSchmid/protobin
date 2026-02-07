use crate::decode::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireDecoder<'a> {
    pub data: &'a [u8],
}

impl<'a> WireDecoder<'a> {
    pub fn new(data: &[u8]) -> WireDecoder<'_> {
        WireDecoder { data }
    }

    /// Read a VARINT uint32 from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_var_uint32(&mut self) -> Result<u32, DecodeError> {
        let mut result = 0u32;
        for i in 0..5 {
            let v = self.take_byte()?;
            result |= ((v & 0b0111_1111) as u32) << (7 * i);
            if v & 0b1000_0000 == 0 {
                return Ok(result);
            }
        }
        Err(DecodeError::UnexpectedContinuationBit)
    }

    /// Read a VARINT uint64 from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_var_uint64(&mut self) -> Result<u64, DecodeError> {
        let mut result = 0u64;
        for i in 0..10 {
            let v = self.take_byte()?;
            result |= ((v & 0b0111_1111) as u64) << (7 * i);
            if v & 0b1000_0000 == 0 {
                return Ok(result);
            }
        }
        Err(DecodeError::UnexpectedContinuationBit)
    }

    /// Read fixed sized [`u32`] from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_fixed32(&mut self) -> Result<u32, DecodeError> {
        self.take_4bytes().map(u32::from_le_bytes)
    }

    /// Read fixed sized [`u64`] from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_fixed64(&mut self) -> Result<u64, DecodeError> {
        self.take_8bytes().map(u64::from_le_bytes)
    }

    /// Read double from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_float(&mut self) -> Result<f32, DecodeError> {
        self.take_4bytes().map(f32::from_le_bytes)
    }

    /// Read double from the `data` slice and move `data`
    /// by the amount of read bytes.
    pub fn read_double(&mut self) -> Result<f64, DecodeError> {
        self.take_8bytes().map(f64::from_le_bytes)
    }

    #[inline]
    pub fn take_byte(&mut self) -> Result<u8, DecodeError> {
        if self.data.is_empty() {
            Err(DecodeError::Len)
        } else {
            let value = unsafe { *self.data.get_unchecked(0) };
            self.data = unsafe {
                core::slice::from_raw_parts(self.data.as_ptr().add(1), self.data.len() - 1)
            };
            Ok(value)
        }
    }

    #[inline]
    pub fn take_4bytes(&mut self) -> Result<[u8; 4], DecodeError> {
        if self.data.len() < 4 {
            Err(DecodeError::Len)
        } else {
            let result = unsafe {
                [
                    *self.data.get_unchecked(0),
                    *self.data.get_unchecked(1),
                    *self.data.get_unchecked(2),
                    *self.data.get_unchecked(3),
                ]
            };
            self.data = unsafe {
                core::slice::from_raw_parts(self.data.as_ptr().add(4), self.data.len() - 4)
            };
            Ok(result)
        }
    }

    #[inline]
    pub fn take_8bytes(&mut self) -> Result<[u8; 8], DecodeError> {
        if self.data.len() < 8 {
            Err(DecodeError::Len)
        } else {
            let result = unsafe {
                [
                    *self.data.get_unchecked(0),
                    *self.data.get_unchecked(1),
                    *self.data.get_unchecked(2),
                    *self.data.get_unchecked(3),
                    *self.data.get_unchecked(4),
                    *self.data.get_unchecked(5),
                    *self.data.get_unchecked(6),
                    *self.data.get_unchecked(7),
                ]
            };
            self.data = unsafe {
                core::slice::from_raw_parts(self.data.as_ptr().add(8), self.data.len() - 8)
            };
            Ok(result)
        }
    }

    #[inline]
    pub fn take_nbyte(&mut self, n: usize) -> Result<&'a [u8], DecodeError> {
        if self.data.len() < n {
            Err(DecodeError::Len)
        } else {
            let result = unsafe { core::slice::from_raw_parts(self.data.as_ptr(), n) };
            self.data = unsafe {
                core::slice::from_raw_parts(self.data.as_ptr().add(n), self.data.len() - n)
            };
            Ok(result)
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::wire::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_var_u32(value in any::<u32>()) {
            let mut writer = WireEncoder::new();
            writer.add_var_uint32(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_var_uint32());
        }
    }

    proptest! {
        #[test]
        fn test_var_u64(value in any::<u64>()) {
            let mut writer = WireEncoder::new();
            writer.add_var_uint64(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_var_uint64());
        }
    }

    proptest! {
        #[test]
        fn test_fixed32(value in any::<u32>()) {
            let mut writer = WireEncoder::new();
            writer.add_fixed32(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_fixed32());
        }
    }

    proptest! {
        #[test]
        fn test_fixed64(value in any::<u64>()) {
            let mut writer = WireEncoder::new();
            writer.add_fixed64(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_fixed64());
        }
    }

    proptest! {
        #[test]
        fn test_read_float(value in any::<f32>()) {
            let mut writer = WireEncoder::new();
            writer.add_float(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_float());
        }
    }

    proptest! {
        #[test]
        fn test_read_double(value in any::<f64>()) {
            let mut writer = WireEncoder::new();
            writer.add_double(value);
            let buf = writer.take_buf();
            let mut reader = WireDecoder::new(&buf);
            prop_assert_eq!(Ok(value), reader.read_double());
        }
    }
}
