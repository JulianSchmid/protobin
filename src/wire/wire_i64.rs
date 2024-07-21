/// Protobuf I64 wire type (fixed 64 bit values) used to
/// encode the Protobuf types `fixed64`, `sfixed64` and `double`.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct WireI64(pub u64);

impl WireI64 {
    /// Interpret the i64 as a Protobuf `fixed64`.
    #[inline]
    pub fn as_fixed64(&self) -> u64 {
        self.0
    }

    /// Interpret the i64 as a Protobuf `sfixed64`.
    #[inline]
    pub fn as_sfixed64(&self) -> i64 {
        i64::from_ne_bytes(self.0.to_ne_bytes())
    }

    /// Interpret the i64 as a Protobuf `double`.
    #[inline]
    pub fn as_double(&self) -> f64 {
        f64::from_ne_bytes(self.0.to_ne_bytes())
    }
}

#[cfg(test)]
mod test {
    use crate::wire::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn as_fixed64(
            value in any::<u64>()
        ) {
            prop_assert_eq!(WireI64(value).as_fixed64(), value);
        }
    }

    proptest! {
        #[test]
        fn as_sfixed64(
            value in any::<u64>()
        ) {
            prop_assert_eq!(
                WireI64(value).as_sfixed64(),
                i64::from_ne_bytes(value.to_ne_bytes())
            );
        }
    }

    proptest! {
        #[test]
        fn as_double(
            value in any::<u64>()
        ) {
            prop_assert_eq!(
                WireI64(value).as_double().to_ne_bytes(),
                f64::from_ne_bytes(value.to_ne_bytes()).to_ne_bytes()
            );
        }
    }
}
