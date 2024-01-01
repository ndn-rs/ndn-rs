use super::*;

#[derive(Clone, Debug)]
pub struct Generic {
    pub r#type: Type,
    pub length: VarNumber,
    pub value: BytesMut,
}

impl Generic {
    fn decode_from_buf<B>(src: &mut B) -> Option<(Type, VarNumber, usize, usize)>
    where
        B: Buf,
    {
        let buf_length = src.remaining();
        let r#type = Type::from_buf(src)?;
        let length = VarNumber::from_buf(src)?;
        let value_length = length.to_usize();
        let value_offset = buf_length - src.remaining();
        if src.remaining() >= value_length {
            Some((r#type, length, value_offset, value_length))
        } else {
            None
        }
    }

    pub fn from_bytes_mut(src: &mut BytesMut) -> Option<Self> {
        let mut src = io::Cursor::new(src);
        let (r#type, length, value_offset, value_length) = Self::decode_from_buf(&mut src)?;
        let src = src.into_inner();
        let _type_and_length = src.split_to(value_offset);
        let value = src.split_to(value_length);
        Some(Self {
            r#type,
            length,
            value,
        })
    }

    pub fn from_bytes(src: &mut Bytes) -> Option<Self> {
        let mut src = io::Cursor::new(src);
        let (r#type, length, value_offset, value_length) = Self::decode_from_buf(&mut src)?;
        let src = src.into_inner();
        let _type_and_length = src.split_to(value_offset);
        let data = src.split_to(value_length);
        let mut value = BytesMut::with_capacity(value_length);
        value.extend_from_slice(&data);
        Some(Self {
            r#type,
            length,
            value,
        })
    }

    pub fn from_slice(src: &[u8]) -> Option<Self> {
        let mut src = io::Cursor::new(src);
        let (r#type, length, value_offset, value_length) = Self::decode_from_buf(&mut src)?;
        let src = src.into_inner();
        let data = src.split_at(value_offset).1.split_at(value_length).0;
        let mut value = BytesMut::with_capacity(value_length);
        value.extend_from_slice(data);
        Some(Self {
            r#type,
            length,
            value,
        })
    }

    pub fn items(self) -> Option<Vec<Self>> {
        let mut items = vec![];
        let mut src = self.value;
        while !src.is_empty() {
            let item = Self::from_bytes_mut(&mut src)?;
            items.push(item)
        }
        Some(items)
    }

    pub fn check_type(self, r#type: Type) -> Result<Self, DecodeError> {
        if self.r#type == r#type {
            Ok(self)
        } else {
            Err(DecodeError::r#type(r#type, self))
        }
    }

    pub fn _self_check_length(self) -> Result<Self, DecodeError> {
        let expected = self.value.len();
        self.check_length(expected)
    }

    pub fn check_length(self, expected: usize) -> Result<Self, DecodeError> {
        let found = self.length.to_usize();
        (found == expected)
            .then_some(self)
            .ok_or(DecodeError::length_mismatch(expected, found))
    }

    pub fn try_into_generic_array_inefficient<T>(self) -> Result<GenericArray<u8, T>, DecodeError>
    where
        T: generic_array::ArrayLength,
    {
        GenericArray::try_from_slice(&self.value)
            .map(|array| array.clone())
            .map_err(|_| DecodeError::length_mismatch(T::to_usize(), self.value.len()))
    }

    pub fn from_tlv<T>(t: T) -> Result<Self, <T as Tlv>::Error>
    where
        T: Tlv,
    {
        let r#type = t.r#type();
        let length = t.length();
        let mut value = BytesMut::with_capacity(length);
        t.encode_value(&mut value)?;
        let length = length.into();
        Ok(Self {
            r#type,
            length,
            value,
        })
    }
}

impl fmt::Display for Generic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Generic")
            .field("type", &self.r#type)
            .field("length", &self.length.to_u64())
            .field("value", &format!("{} octets", self.value.len()))
            .finish_non_exhaustive()
    }
}

impl Tlv for Generic {
    type Error = DecodeError;

    fn r#type(&self) -> Type {
        self.r#type
    }

    fn length(&self) -> usize {
        self.length.to_usize()
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(&self.value);
        Ok(())
    }

    fn decode_value(r#type: Type, length: usize, src: &mut BytesMut) -> Result<Self, Self::Error> {
        let _ = (r#type, length, src);
        todo!("This probably should never be implemented")
    }
}

pub trait OptionGeneric {
    fn invalid_data(self, reason: impl Into<String>) -> Result<Generic, DecodeError>;
}

impl OptionGeneric for Option<Generic> {
    fn invalid_data(self, reason: impl Into<String>) -> Result<Generic, DecodeError> {
        self.ok_or_else(|| DecodeError::invalid(reason))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const P1: &[u8] = &[
        6, 253, 1, 27, 7, 46, 8, 9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 8, 3, 110, 102, 100,
        8, 6, 115, 116, 97, 116, 117, 115, 8, 7, 103, 101, 110, 101, 114, 97, 108, 54, 8, 0, 0, 1,
        140, 67, 46, 112, 8, 50, 1, 0, 20, 9, 25, 2, 3, 232, 26, 3, 50, 1, 0, 21, 85, 128, 18, 50,
        50, 46, 49, 50, 45, 51, 51, 45, 103, 101, 50, 55, 55, 102, 56, 98, 57, 129, 8, 0, 0, 1,
        140, 51, 165, 241, 48, 130, 8, 0, 0, 1, 140, 67, 46, 112, 8, 131, 1, 11, 132, 1, 2, 133, 1,
        2, 134, 1, 0, 135, 1, 2, 144, 2, 20, 191, 145, 2, 3, 230, 151, 1, 0, 146, 2, 20, 191, 147,
        2, 3, 211, 152, 1, 0, 153, 2, 3, 211, 154, 2, 16, 230, 22, 63, 27, 1, 3, 28, 58, 7, 56, 8,
        9, 108, 111, 99, 97, 108, 104, 111, 115, 116, 8, 7, 100, 97, 101, 109, 111, 110, 115, 8, 3,
        110, 102, 100, 8, 3, 75, 69, 89, 8, 8, 236, 225, 196, 116, 178, 135, 206, 56, 8, 4, 115,
        101, 108, 102, 54, 8, 0, 0, 1, 140, 16, 9, 111, 157, 23, 70, 48, 68, 2, 32, 27, 43, 29,
        209, 53, 118, 16, 115, 224, 250, 31, 15, 92, 109, 138, 64, 162, 142, 57, 116, 130, 238,
        247, 33, 230, 126, 27, 122, 198, 27, 212, 30, 2, 32, 49, 242, 60, 6, 201, 70, 78, 10, 105,
        155, 243, 234, 81, 99, 45, 95, 155, 148, 108, 107, 150, 54, 206, 64, 36, 21, 71, 250, 100,
        63, 254, 121, 0,
    ];

    #[test]
    fn decode_and_advance() {
        let mut src = Bytes::from_static(P1);
        assert_eq!(src.remaining(), 288);
        let p = Generic::from_bytes(&mut src).unwrap();
        assert_eq!(src.remaining(), 1);
        assert_eq!(p.r#type, Type::Data);
        assert_eq!(p.length, 283_u64);
    }
}
