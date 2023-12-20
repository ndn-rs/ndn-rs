use super::*;

pub trait Tlv: fmt::Debug + Sized {
    type Error: From<io::Error> + StdError + Send + Sync;

    /// Report this TLV-TYPE as `Type`
    fn r#type(&self) -> Type;

    /// Report TLV-LENGTH as usize
    fn length(&self) -> usize;

    /// Encode the value into the supplied buffer
    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error>;

    /// Decode this object from supplied buffer
    fn decode_value(src: &mut BytesMut) -> Result<Self, Self::Error>;

    /// Encode TLV-TYPE
    fn encode_type(&self, dst: &mut BytesMut) {
        self.r#type().to_varnumber().encode(dst)
    }

    /// Encode TLV_LENGTH
    fn encode_length(&self, dst: &mut BytesMut) {
        VarNumber::from(self.length()).encode(dst)
    }

    /// Report TLV-VALUE as `Bytes` buffer (if value is present)
    fn value(&self) -> Option<Bytes> {
        let length = self.length();
        if length > 0 {
            let mut dst = BytesMut::with_capacity(length);
            self.encode_value(&mut dst).ok()?;
            Some(dst.freeze())
        } else {
            None
        }
    }

    /// Report the total size of the packet or TLV element, including the TLV-TYPE and TLV-LENGTH
    fn size(&self) -> usize {
        let length = self.length();
        self.r#type().to_varnumber().len() + VarNumber::from(length).len() + length
    }
}

impl<T> TlvCodec for T
where
    T: Tlv,
{
    type Error = <T as Tlv>::Error;

    fn total_size(&self) -> usize {
        self.size()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        // let r#type = self.r#type().to_varnumber();
        // let length = self.length();
        // let var_number_length = VarNumber::from(length);
        // let additional = r#type.len() + var_number_length.len() + length;

        // dst.reserve(additional);
        // r#type.encode(dst);
        // var_number_length.encode(dst);
        self.encode_type(dst);
        self.encode_length(dst);
        self.encode_value(dst)
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        // TODO: Need to decode and verify TLV-TYPE and TLV-LENGTH
        T::decode_value(src)
    }
}
