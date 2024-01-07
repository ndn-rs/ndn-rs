use super::*;

pub trait Tlv: fmt::Debug + Sized {
    type Error: From<io::Error> + Into<io::Error> + StdError + Send + Sync;
    const TYPE: Type;

    /// Report this TLV-TYPE as `Type`
    fn r#type(&self) -> Type {
        Self::TYPE
    }

    /// Report TLV-LENGTH as usize
    fn length(&self) -> usize;

    /// Encode TLV-TYPE
    fn encode_type(&self, dst: &mut BytesMut) {
        self.r#type().to_varnumber().encode(dst)
    }

    /// Encode TLV-LENGTH
    fn encode_length(&self, dst: &mut BytesMut) {
        VarNumber::from(self.length()).encode(dst)
    }

    /// Encode the value into the supplied buffer
    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error>;

    // /// Decode TLV-TYPE
    // fn decode_type(src: &mut BytesMut) -> Result<Type, Self::Error>;

    // /// Decode TLV-LENGTH
    // fn decode_length(src: &mut BytesMut) -> Result<VarNumber, Self::Error>;

    /// Decode this object from supplied buffer
    fn decode_value(r#type: Type, length: usize, src: &mut BytesMut) -> Result<Self, Self::Error>;

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
    const TYPE: Type = T::TYPE;

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
        let r#type = Type::decode(src).ok_or_else(|| io::Error::other("Invalid TLV-TYPE"))?;
        let length = VarNumber::decode(src)
            .ok_or_else(|| io::Error::other("Invalid TLV-LENGTH"))?
            .to_usize();
        tracing::trace!(%r#type, length, src_len = src.len(), "Decoding");
        let mut src = src.split_to(length);
        T::decode_value(r#type, length, &mut src)
    }
}
