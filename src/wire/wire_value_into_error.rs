use crate::wire::*;
use core::fmt;

/// Error when it is not possible to convert a [`WireValueRef`]
/// to a specific Protobuf type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WireValueIntoError {
    /// Error if a [`WireValueRef`] has a different wire type than the
    /// requested Protobuf type is encoded with.
    UnexpectedType {
        /// Wire type the requested Protobuf type is encoded with.
        expected: WireType,
        /// Wire type of the value.
        actual: WireType,
    },

    /// Error if the value is too big for a 32 bit value.
    VarIntValueTooBigFor32Bit(u64),
}

impl fmt::Display for WireValueIntoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use WireValueIntoError::*;
        match self {
            UnexpectedType { expected, actual } => write!(
                f,
                "Wire value conversion error: Expected wire type '{expected:?}' but got '{actual:?}'."
            ),
            VarIntValueTooBigFor32Bit(value) => write!(
                f,
                "Wire value conversion error: Varint value '{value}' is too big for 32 bits."
            ),
        }
    }
}

impl core::error::Error for WireValueIntoError {}

#[cfg(test)]
mod test {
    use super::*;
    use core::error::Error;
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn debug() {
        assert_eq!(
            "UnexpectedType { expected: VarInt, actual: I64 }",
            format!(
                "{:?}",
                WireValueIntoError::UnexpectedType {
                    expected: WireType::VarInt,
                    actual: WireType::I64,
                }
            )
        );
    }

    #[test]
    fn clone_eq_hash() {
        let err = WireValueIntoError::VarIntValueTooBigFor32Bit(u64::MAX);
        assert_eq!(err, err.clone());
        let hash_a = {
            let mut hasher = DefaultHasher::new();
            err.hash(&mut hasher);
            hasher.finish()
        };
        let hash_b = {
            let mut hasher = DefaultHasher::new();
            err.clone().hash(&mut hasher);
            hasher.finish()
        };
        assert_eq!(hash_a, hash_b);
    }

    #[test]
    fn fmt() {
        use WireValueIntoError::*;
        assert_eq!(
            "Wire value conversion error: Expected wire type 'VarInt' but got 'Len'.",
            format!(
                "{}",
                UnexpectedType {
                    expected: WireType::VarInt,
                    actual: WireType::Len,
                }
            )
        );
        assert_eq!(
            "Wire value conversion error: Varint value '4294967296' is too big for 32 bits.",
            format!("{}", VarIntValueTooBigFor32Bit(1 << 32))
        );
    }

    #[test]
    fn source() {
        use WireValueIntoError::*;
        let values = [
            UnexpectedType {
                expected: WireType::I32,
                actual: WireType::I64,
            },
            VarIntValueTooBigFor32Bit(1 << 32),
        ];
        for v in values {
            assert!(v.source().is_none());
        }
    }
}
