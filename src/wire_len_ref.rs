use core::str::Utf8Error;

/// LEN wire type that can be interpred as `string`, `bytes`,
/// "embedded messages" and "packed repeated fields" Prototype
/// types.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireLenRef<'a> {
    pub data: &'a [u8],
}

impl<'a> WireLenRef<'a> {
    #[inline]
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }

    #[inline]
    pub fn try_as_string(&self) -> Result<&str, Utf8Error> {
        core::str::from_utf8(&self.data)
    }
}
