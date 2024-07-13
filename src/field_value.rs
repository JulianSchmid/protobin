use crate::*;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldValue<'a> {
    pub field_number: FieldNumber,
    pub value: WireValueRef<'a>,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct FieldNumber(u32);
