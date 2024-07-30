use std::ops::RangeInclusive;


#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldNumber(pub(crate) u32);

impl FieldNumber {

    /// Maximum allowed field number.
    /// 
    /// This is determined by the fact that the field number is
    /// part of the tag and the tag is a u32 composed of a 3 bit
    /// value for the "wire type" and 29 bit for the "field number".
    pub const MAX_ALLOWED_U32: u32 = u32::MAX >> 3;

    /// Range of field values reserved for protobuf internal usage.
    pub const RESERVED_FOR_PROTO_INTERNAL: RangeInclusive<u32> = 19000..=19999;

    /// Converts a given [`u32`] to a [`FieldNumber`] as long as it is smaller or
    /// equal `0x1FFFFFFF` ([`FieldNumber::MAX_ALLOWED_U32`]) and at least `1` or
    /// bigger. Otherwise an error is returned.
    #[inline]
    pub const fn try_from_u32(value: u32) -> Result<FieldNumber, FieldNumberIntoError> {
        if value < 1 || value > (u32::MAX >> 3) {
            Err(FieldNumberIntoError{
                invalid_value: value
            })
        } else {
            Ok(FieldNumber(value))
        }
    }

    /// Convert an [`u32`] to a field number without checking it is smaller
    /// or equal [`FieldNumber::MAX_ALLOWED_U32`] and at least `1`.
    /// 
    /// # Safety
    /// 
    /// The caller must ensure the provided value is smaller or equal
    /// [`FieldNumber::MAX_ALLOWED_U32`] and at least `1`. If not undefined
    /// behavior will occur when the field number is serialized.
    /// In debug an assertion will panic the program if a value greater then
    /// [`FieldNumber::MAX_ALLOWED_U32`] is passed as argument.
    #[inline]
    pub const unsafe fn from_u32_unchecked(value: u32) -> FieldNumber {
        debug_assert!(value <= (u32::MAX >> 3));
        debug_assert!(1 <= value);
        FieldNumber(value)
    }

    #[inline]
    pub const fn value(&self) -> u32 {
        self.0
    }

    /// Return the next unreserved FieldNumber.
    ///
    /// Panics if the field number exceeds `0x1FFFFFFF`
    /// ([`FieldNumber::MAX_ALLOWED_U32`]).
    #[inline]
    pub const fn get_next_unreserved(&self) -> FieldNumber {
        assert!(self.0 <= FieldNumber::MAX_ALLOWED_U32);
        if 19000 <= self.0 + 1 && self.0 < 19999 {
            // According to https://protobuf.dev/programming-guides/proto3/#assigning
            // 19000 till 19999 are reserved, jump after these
            FieldNumber(20000)
        } else {
            FieldNumber(self.0 + 1)
        }
    }
}

impl TryFrom<u32> for FieldNumber {
    type Error = FieldNumberIntoError;

    #[inline]
    fn try_from(value: u32) -> Result<Self, Self::Error> {
        FieldNumber::try_from_u32(value)
    }
}

impl From<FieldNumber> for u32 {
    #[inline]
    fn from(value: FieldNumber) -> Self {
        value.0
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldNumberIntoError {
    invalid_value: u32,
}
