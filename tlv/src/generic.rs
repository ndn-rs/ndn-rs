use super::*;

#[derive(Clone, Debug)]
pub struct Generic {
    pub r#type: Type,
    pub length: VarNumber,
    pub value: Bytes,
}

impl Generic {
    pub fn from_slice(mut src: &[u8]) -> Option<Self> {
        Self::from_buf(&mut src)
    }

    pub fn from_buf<B>(src: &mut B) -> Option<Self>
    where
        B: Buf,
    {
        let r#type = Type::from_buf(src)?;
        let length = VarNumber::from_buf(src)?;
        let value_size = length.to_u64() as usize;
        let value = (src.remaining() >= value_size).then(|| src.copy_to_bytes(value_size))?;
        Some(Self {
            r#type,
            length,
            value,
        })
    }

    pub fn items(&self) -> Option<Vec<Self>> {
        let mut items = vec![];
        let mut src = self.value.as_ref();
        while !src.is_empty() {
            let item = Self::from_buf(&mut src)?;
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

    pub fn self_check_length(self) -> Result<Self, DecodeError> {
        let length = self.value.len();
        self.check_length(length)
    }

    pub fn check_length(self, length: usize) -> Result<Self, DecodeError> {
        if self.length == length as u64 {
            Ok(self)
        } else {
            Err(DecodeError::LengthMismatch(self))
        }
    }

    pub fn try_into_generic_array<T>(self) -> Result<GenericArray<u8, T>, DecodeError>
    where
        T: generic_array::ArrayLength,
    {
        GenericArray::try_from_slice(&self.value)
            .map(|array| array.clone())
            .map_err(|_| DecodeError::LengthMismatch(self))
    }

    pub fn from_tlv<T>(t: T) -> Result<Self, <T as Tlv>::Error>
    where
        T: Tlv,
    {
        let r#type = t.r#type();
        let length = t.length();
        let mut value = BytesMut::with_capacity(length);
        t.encode_value(&mut value)?;
        let value = value.freeze();
        let length = length.into();
        Ok(Self {
            r#type,
            length,
            value,
        })
    }
}

impl Tlv for Generic {
    type Error = DecodeError;

    fn r#type(&self) -> Type {
        self.r#type
    }

    fn length(&self) -> usize {
        self.length.to_u64() as usize
    }

    fn encode_value(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(&self.value);
        Ok(())
    }

    fn decode_value(_src: &mut BytesMut) -> Result<Self, Self::Error> {
        todo!("This probably should never be implemented")
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
        let p = Generic::from_buf(&mut src).unwrap();
        assert_eq!(src.remaining(), 1);
        assert_eq!(p.r#type, Type::Data);
        assert_eq!(p.length, 283_u64);
    }
}