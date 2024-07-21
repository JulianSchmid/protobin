use crate::{builders::*, wire::*, FieldNumber};

/// Buffers to serialize a message (should be reused as much as possible
/// to avoid allocations).
/// 
/// The [`MsgEncBuf`] contains the needed buffers to encode
/// a protobuf message to its "on the wire" binary format.
/// 
/// The encoding is done in two stages:
/// 
/// 1. Determine the "length" values used in the message & it's submessages.
/// 2. Serialize the data
/// 
/// # Why are two stages needed?
/// 
/// Step one is needed as every submessage, repeated elements & packed values
/// start with their "byte length". In a simpler format one could simply
/// adding a placeholder value for the "byte length" and fill in the length
/// after the serialisation of conained elements is done and the length known.
/// 
/// The issue is that the "byte length" itself is encoded as a "varint" which
/// means the length itself can take between 1-10 bytes and therefor the entire
/// serialized data would need to be shifted around based on its length.
/// 
/// To avoid this all the length values are determined first and safed so they
/// can be used in the second serialisation step.
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub struct MsgBuilder {
    pub(crate) lens: Vec<(FieldNumber,i32)>,
    pub(crate) len_stack: Vec<LenStackEntry>,
    pub(crate) encoder: WireEncoder,
}

impl MsgBuilder {
    /// Start encoding a new message encoding (returns length builder as step 1).
    pub fn start<'a>(&'a mut self, num_len_values: Option<usize>) -> MsgLenBuilder<'a> {
        // clear all previous data
        self.lens.clear();
        self.len_stack.clear();
        self.encoder.buf.clear();

        // reserve enough memory in case we get a upper limit of len values
        if let Some(num_len_values) = num_len_values {
            self.lens.reserve(num_len_values);
            self.len_stack.reserve(num_len_values);
        }

        // return the len builder for step 1
        MsgLenBuilder{
            buf: self,
            cur_len: 0,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub(crate) struct LenStackEntry {
    pub(crate) len: i32,
    pub(crate) t: LenStackType,
    pub(crate) len_index: usize,
    pub(crate) field_number: FieldNumber,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub(crate) enum LenStackType {
    Msg,
    Packed,
}
