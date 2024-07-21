
/// Helper to calculate the number of bytes used when encoding a
/// given set of values.a
#[derive(Default, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireLenCalc {
    /// Number of bytes needed to encode the provided values.
    pub len: usize,
}

impl WireLenCalc {
    pub fn new() -> WireLenCalc {
        WireLenCalc{ len: 0 }
    }

    /// Add the needed len to encode the given [`u32`] as VARINT to the overall `len`.
    pub fn add_var_uint32(&mut self, value: u32) {
        // determine needed byte length
        if value < (1 << 7) {
            self.len += 1;
        } else if value < (1 << (7 * 2)) {
            self.len += 2;
        } else if value < (1 << (7 * 3)) {
            self.len += 3;
        } else if value < (1 << (7 * 4)) {
            self.len += 4;
        } else {
            self.len += 5;
        }
    }

    /// Add the needed len to encode the given [`u64`] as VARINT to the overall `len`.
    pub fn add_var_uint64(&mut self, value: u64) {
        // determine needed byte length
        if value < (1 << 7) {
            self.len += 1;
        } else if value < (1 << (7 * 2)) {
            self.len += 2;
        } else if value < (1 << (7 * 3)) {
            self.len += 3;
        } else if value < (1 << (7 * 4)) {
            self.len += 4;
        } else if value < (1 << (7 * 5)) {
            self.len += 5;
        } else if value < (1 << (7 * 6)) {
            self.len += 6;
        } else if value < (1 << (7 * 7)) {
            self.len += 7;
        } else if value < (1 << (7 * 8)) {
            self.len += 8;
        } else if value < (1 << (7 * 9)) {
            self.len += 9;
        } else {
            self.len += 10;
        }
    }

    /// Add the needed len to encode a fixed32 to the overall `len`.
    pub fn add_fixed32(&mut self) {
        self.len += 4;
    }

    /// Add the needed len to encode a fixed64 to the overall `len`.
    pub fn add_fixed64(&mut self) {
        self.len += 8;
    }

    /// Add the needed len to encode a float to the overall `len`.
    pub fn add_float(&mut self) {
        self.len += 4;
    }

    /// Add the needed len to encode a double to the overall `len`.
    pub fn add_double(&mut self) {
        self.len += 8;
    }

    /// Add the length needed to encode bytes of the given length.
    pub fn add_bytes(&mut self, len: usize) {
        self.len += len;
    }
}
