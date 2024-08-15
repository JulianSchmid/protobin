#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireEncoder {
    /// Buffer where the encoded bytes will be written to.
    pub buf: Vec<u8>,
}

impl WireEncoder {
    /// Create a [`ProtobufEncoder`] with a new `Vec`.
    pub fn new() -> WireEncoder {
        WireEncoder { buf: Vec::new() }
    }

    /// Create a [`ProtobufEncoder`] with the given buffer.
    pub fn with_buf(buf: Vec<u8>) -> WireEncoder {
        WireEncoder { buf }
    }

    /// Add the given [`u32`] as VARINT to the `buf`.
    pub fn add_var_uint32(&mut self, value: u32) {
        // determine needed byte length
        if value < (1 << 7) {
            // 1 byte
            self.buf.push(value as u8);
        } else if value < (1 << (7 * 2)) {
            // 2 bytes
            self.buf
                .extend_from_slice(&[0b1000_0000 | (value as u8), (value >> 7) as u8]);
        } else if value < (1 << (7 * 3)) {
            // 3 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                ((value >> (7 * 2)) as u8),
            ]);
        } else if value < (1 << (7 * 4)) {
            // 4 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                ((value >> (7 * 3)) as u8),
            ]);
        } else {
            // 5 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                ((value >> (7 * 4)) as u8),
            ]);
        }
    }

    /// Add a protobuf `int32` to the `buf` encoded as VARINT.
    #[inline]
    pub fn add_var_int32(&mut self, value: i32) {
        self.add_var_uint32(u32::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Add a protobuf `sint32` to the `buf` encoded as VARINT with zig zag encoding.
    #[inline]
    pub fn add_var_sint32(&mut self, value: i32) {
        let zigzag = (value << 1) ^ (value >> 31);
        self.add_var_uint32(u32::from_ne_bytes(zigzag.to_ne_bytes()))
    }

    /// Add the given [`u64`] as VARINT to the `buf`.
    pub fn add_var_uint64(&mut self, value: u64) {
        // determine needed byte length
        if value < (1 << 7) {
            // 1 byte
            self.buf.push(value as u8);
        } else if value < (1 << (7 * 2)) {
            // 2 bytes
            self.buf
                .extend_from_slice(&[0b1000_0000 | (value as u8), (value >> 7) as u8]);
        } else if value < (1 << (7 * 3)) {
            // 3 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                ((value >> (7 * 2)) as u8),
            ]);
        } else if value < (1 << (7 * 4)) {
            // 4 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                ((value >> (7 * 3)) as u8),
            ]);
        } else if value < (1 << (7 * 5)) {
            // 5 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                ((value >> (7 * 4)) as u8),
            ]);
        } else if value < (1 << (7 * 6)) {
            // 6 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                0b1000_0000 | ((value >> (7 * 4)) as u8),
                ((value >> (7 * 5)) as u8),
            ]);
        } else if value < (1 << (7 * 7)) {
            // 7 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8 & 0b0111_1111),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                0b1000_0000 | ((value >> (7 * 4)) as u8),
                0b1000_0000 | ((value >> (7 * 5)) as u8),
                ((value >> (7 * 6)) as u8),
            ]);
        } else if value < (1 << (7 * 8)) {
            // 8 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                0b1000_0000 | ((value >> (7 * 4)) as u8),
                0b1000_0000 | ((value >> (7 * 5)) as u8),
                0b1000_0000 | ((value >> (7 * 6)) as u8),
                ((value >> (7 * 7)) as u8),
            ]);
        } else if value < (1 << (7 * 9)) {
            // 9 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                0b1000_0000 | ((value >> (7 * 4)) as u8),
                0b1000_0000 | ((value >> (7 * 5)) as u8),
                0b1000_0000 | ((value >> (7 * 6)) as u8),
                0b1000_0000 | ((value >> (7 * 7)) as u8),
                ((value >> (7 * 8)) as u8),
            ]);
        } else {
            // 10 bytes
            self.buf.extend_from_slice(&[
                0b1000_0000 | (value as u8),
                0b1000_0000 | ((value >> 7) as u8),
                0b1000_0000 | ((value >> (7 * 2)) as u8),
                0b1000_0000 | ((value >> (7 * 3)) as u8),
                0b1000_0000 | ((value >> (7 * 4)) as u8),
                0b1000_0000 | ((value >> (7 * 5)) as u8),
                0b1000_0000 | ((value >> (7 * 6)) as u8),
                0b1000_0000 | ((value >> (7 * 7)) as u8),
                0b1000_0000 | ((value >> (7 * 8)) as u8),
                ((value >> (7 * 9)) as u8),
            ]);
        }
    }

    /// Add a protobuf `int64` to the `buf` encoded as VARINT.
    #[inline]
    pub fn add_var_int64(&mut self, value: i64) {
        self.add_var_uint64(u64::from_ne_bytes(value.to_ne_bytes()))
    }

    /// Add a protobuf `sint64` to the `buf` encoded as VARINT with zig zag encoding.
    #[inline]
    pub fn add_var_sint64(&mut self, value: i64) {
        let zigzag = (value << 1) ^ (value >> 63);
        self.add_var_uint64(u64::from_ne_bytes(zigzag.to_ne_bytes()))
    }

    /// Add a protobuf `bool` to the `buf` encoded as VARINT.
    #[inline]
    pub fn add_bool(&mut self, value: bool) {
        self.buf.push(if value { 1 } else { 0 });
    }

    /// Add the given [`u32`] as `fixed32`` to the `buf`.
    #[inline]
    pub fn add_fixed32(&mut self, value: u32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Add the given [`i32`] as `sfixed32`` to the `buf`.
    #[inline]
    pub fn add_sfixed32(&mut self, value: i32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Add the given [`u64`] as fixed64 to the `buf`.
    #[inline]
    pub fn add_fixed64(&mut self, value: u64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Add the given [`i64`] as sfixed64 to the `buf`.
    #[inline]
    pub fn add_sfixed64(&mut self, value: i64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Add the given [`f32`] as float to the `buf`.
    #[inline]
    pub fn add_float(&mut self, value: f32) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Add the given [`f64`] as double to the `buf`.
    #[inline]
    pub fn add_double(&mut self, value: f64) {
        self.buf.extend_from_slice(&value.to_le_bytes());
    }

    /// Destroys the [`ProtobufEncoder`] and returns the `buf` of the destroyed
    /// [`ProtobufEncoder`].
    #[inline]
    pub fn take_buf(self) -> Vec<u8> {
        self.buf
    }
}
