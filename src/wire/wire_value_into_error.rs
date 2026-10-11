use crate::wire::*;

/// Error when it is not possible to convert a [`WireValueRef`]
/// to a specific Protobuf type.
#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WireValueIntoError {
    /// Error if a [`WireValueRef`] is not [`WireValueRef::VarInt`] even though
    /// it was expected to be one.
    UnexpectedType {
        expected: WireType,
        actual: WireType,
    },

    /// Error if the value is too big for a 32 bit value.
    VarIntValueTooBigFor32Bit(u64),
}
