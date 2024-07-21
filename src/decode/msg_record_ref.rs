use crate::{*, wire::*};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MsgRecordRef<'a> {
    pub field_number: FieldNumber,
    pub value: WireValueRef<'a>,
}
