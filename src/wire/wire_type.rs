#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WireType {
    VarInt = 0,
    I64 = 1,
    Len = 2,
    SGroup = 3,
    EGroup = 4,
    I32 = 5,
}
