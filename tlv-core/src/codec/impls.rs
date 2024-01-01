use generic_array::{ArrayLength, GenericArray};

use super::*;

impl<T> TlvCodec for Option<T>
where
    T: TlvCodec,
{
    type Error = T::Error;

    fn total_size(&self) -> usize {
        if let Some(ref item) = self {
            item.total_size()
        } else {
            0
        }
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        if let Some(ref item) = self {
            item.encode(dst)
        } else {
            Ok(())
        }
        // self.as_ref().map_or(Ok(()), |item| item.encode(dst))
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        if src.is_empty() {
            Ok(None)
        } else {
            T::decode(src).map(Some)
        }
    }
}

impl<T> TlvCodec for Vec<T>
where
    T: TlvCodec,
{
    type Error = T::Error;

    fn total_size(&self) -> usize {
        self.iter().map(|item| item.total_size()).sum()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        for item in self {
            item.encode(dst)?;
        }
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let mut items = vec![];
        while !src.is_empty() {
            let item = T::decode(src)?;
            items.push(item);
        }
        Ok(items)
    }
}

impl TlvCodec for u8 {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        1
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_u8(*self);
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        if src.len() == 1 {
            Ok(src.get_u8())
        } else {
            Err(io::Error::other("Must be exactly one byte"))
        }
    }
}

// impl TlvCodec for [u8; 4] {
//     type Error = io::Error;

//     fn total_size(&self) -> usize {
//         self.len()
//     }

//     fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
//         dst.put_slice(self);
//         Ok(())
//     }

//     fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
//         let mut octets = [0; 4];
//         if src.len() == octets.len() {
//             src.copy_to_slice(&mut octets);
//             Ok(octets)
//         } else {
//             Err(io::Error::other("Must have exactly four bytes"))
//         }
//     }
// }

impl<const N: usize> TlvCodec for [u8; N] {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        self.len()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.put_slice(self);
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let mut octets = [0; N];
        if src.len() == octets.len() {
            src.copy_to_slice(&mut octets);
            Ok(octets)
        } else {
            Err(io::Error::other("Must have exactly four bytes"))
        }
    }
}

impl<T> TlvCodec for GenericArray<u8, T>
where
    T: ArrayLength,
{
    type Error = io::Error;

    fn total_size(&self) -> usize {
        Self::len()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(self.len());
        dst.extend(self);
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let octets = src.iter().copied();
        Self::try_from_iter(octets).map_err(|_| io::Error::other("Wrong number of bytes"))
    }
}

impl TlvCodec for String {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        self.len()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(self.len());
        dst.extend(self.as_bytes());
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        let vec = src.to_vec();
        Self::from_utf8(vec).map_err(io::Error::other)
    }
}

impl TlvCodec for Bytes {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        self.len()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(self.len());
        dst.extend(self);
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        Ok(src.copy_to_bytes(src.len()))
    }
}

impl TlvCodec for BytesMut {
    type Error = io::Error;

    fn total_size(&self) -> usize {
        self.len()
    }

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error> {
        dst.reserve(self.len());
        dst.extend(self);
        Ok(())
    }

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error> {
        Ok(src.split())
    }
}
