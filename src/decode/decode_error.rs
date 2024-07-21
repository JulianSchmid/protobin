#[derive(Debug, Clone, Eq, PartialEq)]
pub enum DecodeError {
    Len,
    UnexpectedContinuationBit,
    UnknownWireType(u32),
}
