use core::fmt;

/// Error returned when protobuf bytes can not be decoded.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum DecodeError {
    /// The data ended before a value was complete.
    Len {
        /// Number of bytes needed to read the value.
        required: usize,
        /// Number of bytes that were left in the data.
        remaining: usize,
    },

    /// A varint still had its continuation bit set after the maximum
    /// number of bytes for its type.
    UnexpectedContinuationBit {
        /// Maximum number of bytes allowed for the varint (5 for 32 bit
        /// and 10 for 64 bit values).
        max_bytes: usize,
    },

    /// The tag contained a wire type that is not part of the
    /// [protobuf encoding](https://protobuf.dev/programming-guides/encoding/#structure).
    UnknownWireType(u32),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use DecodeError::*;
        match self {
            Len {
                required,
                remaining,
            } => write!(
                f,
                "Decode error: Not enough data, '{required}' byte(s) required but only '{remaining}' byte(s) left."
            ),
            UnexpectedContinuationBit { max_bytes } => write!(
                f,
                "Decode error: Varint is longer than the allowed '{max_bytes}' bytes."
            ),
            UnknownWireType(wire_type) => {
                write!(f, "Decode error: Unknown wire type '{wire_type}'.")
            }
        }
    }
}

impl core::error::Error for DecodeError {}

#[cfg(test)]
mod test {
    use super::*;
    use core::{
        error::Error,
        hash::{Hash, Hasher},
    };
    use std::collections::hash_map::DefaultHasher;

    #[test]
    fn debug() {
        assert_eq!(
            "Len { required: 4, remaining: 1 }",
            format!(
                "{:?}",
                DecodeError::Len {
                    required: 4,
                    remaining: 1
                }
            )
        );
    }

    #[test]
    fn clone_eq_hash() {
        let err = DecodeError::UnknownWireType(7);
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
        use DecodeError::*;
        assert_eq!(
            "Decode error: Not enough data, '8' byte(s) required but only '3' byte(s) left.",
            format!(
                "{}",
                Len {
                    required: 8,
                    remaining: 3
                }
            )
        );
        assert_eq!(
            "Decode error: Varint is longer than the allowed '5' bytes.",
            format!("{}", UnexpectedContinuationBit { max_bytes: 5 })
        );
        assert_eq!(
            "Decode error: Unknown wire type '7'.",
            format!("{}", UnknownWireType(7))
        );
    }

    #[test]
    fn source() {
        use DecodeError::*;
        let values = [
            Len {
                required: 1,
                remaining: 0,
            },
            UnexpectedContinuationBit { max_bytes: 10 },
            UnknownWireType(6),
        ];
        for v in values {
            assert!(v.source().is_none());
        }
    }
}
