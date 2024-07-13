/// Protobuf I32 wire type (fixed 32 bit values) used to
/// encode the Protobuf types `fixed32`, `sfixed32` and `float`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireI32(pub u32);

impl WireI32 {
    /// Interpret the I32 as a Protobuf `fixed32`.
    #[inline]
    pub fn as_fixed32(&self) -> u32 {
        self.0
    }

    /// Interpret the I32 as a Protobuf `sfixed32`.
    #[inline]
    pub fn as_sfixed32(&self) -> i32 {
        i32::from_ne_bytes(self.0.to_ne_bytes())
    }

    /// Interpret the I32 as a Protobuf `float`.
    #[inline]
    pub fn as_float(&self) -> f32 {
        f32::from_ne_bytes(self.0.to_ne_bytes())
    }
}

#[cfg(test)]
mod test {
    use crate::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn as_fixed32(
            value in any::<u32>()
        ) {
            prop_assert_eq!(WireI32(value).as_fixed32(), value);
        }
    }

    proptest! {
        #[test]
        fn as_sfixed32(
            value in any::<u32>()
        ) {
            prop_assert_eq!(
                WireI32(value).as_sfixed32(),
                i32::from_ne_bytes(value.to_ne_bytes())
            );
        }
    }

    proptest! {
        #[test]
        fn as_double(
            value in any::<u32>()
        ) {
            prop_assert_eq!(
                WireI32(value).as_float().to_ne_bytes(),
                f32::from_ne_bytes(value.to_ne_bytes()).to_ne_bytes()
            );
        }
    }
}
