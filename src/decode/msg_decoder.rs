use crate::{decode::*, wire::*, *};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub struct MsgDecoder<'a> {
    pub wire_decoder: WireDecoder<'a>,
}

impl<'a> MsgDecoder<'a> {
    pub fn new(data: &[u8]) -> MsgDecoder {
        MsgDecoder {
            wire_decoder: WireDecoder { data },
        }
    }

    /// Returns the next message record or TLV (Tag-Length-Value) until
    /// an error is encountered or no more data is present.
    ///
    /// In case an error is encountered the error is returned and in the following
    /// call `None`.
    pub fn next(&mut self) -> Option<Result<MsgRecordRef<'a>, DecodeError>> {
        if self.wire_decoder.data.is_empty() {
            return None;
        }
        match self.next_inner() {
            Err(err) => {
                // invalidate the wire decoder so we don't trigger an error in
                // an infinite loop
                self.wire_decoder.data = &[];
                Some(Err(err))
            }
            other => Some(other),
        }
    }

    fn next_inner(&mut self) -> Result<MsgRecordRef<'a>, DecodeError> {
        // read field number & tag
        let tag = self.wire_decoder.read_var_uint32()?;
        let field_number = FieldNumber(tag >> 3);
        let wire_type = tag & 0b111;
        let value = match wire_type {
            // VARINT
            0 => WireValueRef::VarInt(WireVarInt::from_raw(self.wire_decoder.read_var_uint64()?)),
            // I64
            1 => WireValueRef::I64(WireI64(self.wire_decoder.read_fixed64()?)),
            // LEN
            2 => {
                let len: usize = self.wire_decoder.read_var_uint32()? as usize;
                let data = self.wire_decoder.take_nbyte(len)?;
                WireValueRef::Len(WireLenRef { data })
            }
            // SGROUP
            3 => WireValueRef::SGroup,
            // EGROUP
            4 => WireValueRef::EGroup,
            // I32
            5 => WireValueRef::I32(WireI32(self.wire_decoder.read_fixed32()?)),
            unknown => {
                return Err(DecodeError::UnknownWireType(unknown));
            }
        };
        Ok(MsgRecordRef {
            field_number,
            value,
        })
    }
}
