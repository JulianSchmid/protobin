use crate::wire::*;

/// A "on the wire" protobuf value reference that can be furhter
/// interpreted into a "proto type" based on it's definition.
#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WireValueRef<'a> {
    /// Variable sized integer used for `int32`,
    /// `int64`, `uint32`, `uint64`, `sint32`, `sint64`,
    /// `bool` and `enum`.
    VarInt(WireVarInt),
    /// Fixed size 64bit value used for `fixed64`,
    /// `sfixed64` and `double`.
    I64(WireI64),
    /// LEN wire type that can be interpred as `string`, `bytes`,
    /// "embedded messages" and "packed repeated fields" Prototype
    /// types.
    Len(WireLenRef<'a>),
    SGroup,
    EGroup,
    /// Fixed size 32bit value used for `fixed32`,
    /// `sfixed32` and `float`.
    I32(WireI32),
}

impl<'a> WireValueRef<'a> {
    /// Return the type of wire value (e.g. [`WireValueRef::I64`] will return
    /// [`WireType::I64`]).
    pub fn write_type(&self) -> WireType {
        match &self {
            WireValueRef::VarInt(_) => WireType::VarInt,
            WireValueRef::I64(_) => WireType::I64,
            WireValueRef::Len(_) => WireType::Len,
            WireValueRef::SGroup => WireType::SGroup,
            WireValueRef::EGroup => WireType::EGroup,
            WireValueRef::I32(_) => WireType::I32,
        }
    }

    /// Try to interpret the value as a Protobuf `bool` (varint).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 1 `None` is returned instead.
    #[inline]
    pub fn try_as_bool(&self) -> Result<bool, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            v.try_as_bool().ok_or(VarIntValueTooBigFor32Bit(v.raw()))
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `uint32` (varint).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 32 bits `None` is returned instead.
    #[inline]
    pub fn try_as_uint32(&self) -> Result<u32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            v.try_as_uint32().ok_or(VarIntValueTooBigFor32Bit(v.raw()))
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `int32` (varint,
    /// two's complement encoding).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 32 bits `None` is returned instead.
    #[inline]
    pub fn try_as_int32(&self) -> Result<i32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            v.try_as_int32().ok_or(VarIntValueTooBigFor32Bit(v.raw()))
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `sint32` (varint,
    /// ZigZag encoding).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 32 bits `None` is returned instead.
    #[inline]
    pub fn try_as_sint32(&self) -> Result<i32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            v.try_as_sint32().ok_or(VarIntValueTooBigFor32Bit(v.raw()))
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `uint64` (varint).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 32 bits `None` is returned instead.
    #[inline]
    pub fn try_as_uint64(&self) -> Result<u64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            Ok(v.as_uint64())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `int64` (varint,
    /// two's complement encoding).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 64 bits `None` is returned instead.
    #[inline]
    pub fn try_as_int64(&self) -> Result<i64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            Ok(v.as_int64())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `sint64` (varint,
    /// ZigZag encoding).
    ///
    /// If the value is not a [`WireValueRef::VarInt`] or if the
    /// [`WireVarInt`] contains a value that is bigger then
    /// 64 bits `None` is returned instead.
    #[inline]
    pub fn try_as_sint64(&self) -> Result<i64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::VarInt(v) = &self {
            Ok(v.as_sint64())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `fixed32`.
    ///
    /// If the value is not a [`WireValueRef::I32`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_fixed32(&self) -> Result<u32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I32(v) = &self {
            Ok(v.as_fixed32())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `sfixed32`.
    ///
    /// If the value is not a [`WireValueRef::I32`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_sfixed32(&self) -> Result<i32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I32(v) = &self {
            Ok(v.as_sfixed32())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `float`.
    ///
    /// If the value is not a [`WireValueRef::I32`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_float(&self) -> Result<f32, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I32(v) = &self {
            Ok(v.as_float())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `fixed64`.
    ///
    /// If the value is not a [`WireValueRef::I64`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_fixed64(&self) -> Result<u64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I64(v) = &self {
            Ok(v.as_fixed64())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `sfixed64`.
    ///
    /// If the value is not a [`WireValueRef::I64`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_sfixed64(&self) -> Result<i64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I64(v) = &self {
            Ok(v.as_sfixed64())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }

    /// Try to interpret the value as a Protobuf `double`.
    ///
    /// If the value is not a [`WireValueRef::I64`] `None` is
    /// returned instead.
    #[inline]
    pub fn as_double(&self) -> Result<f64, WireValueIntoError> {
        use WireValueIntoError::*;
        if let WireValueRef::I64(v) = &self {
            Ok(v.as_double())
        } else {
            Err(UnexpectedType {
                expected: WireType::VarInt,
                actual: self.write_type(),
            })
        }
    }
}

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
