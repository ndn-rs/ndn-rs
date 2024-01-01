use std::cmp;
use std::fmt;
use std::hash;
use std::ops;

use bytes::{Buf, BufMut, Bytes, BytesMut};

#[allow(clippy::len_without_is_empty)]
#[derive(Clone, Debug)]
pub struct VarNumber {
    bytes: Bytes,
    value: u64,
}

impl VarNumber {
    pub const fn zero() -> Self {
        let bytes = Bytes::from_static(&[0]);
        let value = 0;
        Self { bytes, value }
    }

    pub const fn one() -> Self {
        let bytes = Bytes::from_static(&[1]);
        let value = 1;
        Self { bytes, value }
    }

    pub const fn two() -> Self {
        let bytes = Bytes::from_static(&[2]);
        let value = 2;
        Self { bytes, value }
    }

    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    pub fn from_u64(value: u64) -> Self {
        let bytes = match value {
            x @ 0..=0xFC => {
                let mut bytes = BytesMut::with_capacity(1);
                bytes.put_u8(x as u8);
                bytes.freeze()
            }
            x @ 0xFD..=0xFFFF => {
                let mut bytes = BytesMut::with_capacity(3);
                bytes.put_u8(0xFD);
                bytes.put_u16(x as u16);
                bytes.freeze()
            }
            x @ 0x1_0000..=0xFFFF_FFFF => {
                let mut bytes = BytesMut::with_capacity(5);
                bytes.put_u8(0xFE);
                bytes.put_u32(x as u32);
                bytes.freeze()
            }
            x @ 0x1_0000_0000..=0xFFFF_FFFF_FFFF_FFFF => {
                let mut bytes = BytesMut::with_capacity(9);
                bytes.put_u8(0xFF);
                bytes.put_u64(x);
                bytes.freeze()
            }
        };
        Self { bytes, value }
    }

    pub fn to_u64(&self) -> u64 {
        self.value
    }

    pub fn to_usize(&self) -> usize {
        self.value as usize
    }

    // Cloning `Bytes` should be simple and relatively inexpensive, as it just creates another
    // reference to the original data.
    pub fn bytes(&self) -> Bytes {
        self.bytes.clone()
    }

    pub fn as_bytes(&self) -> &[u8] {
        &self.bytes
    }

    pub fn from_slice(mut src: &[u8]) -> Option<Self> {
        Self::from_buf(&mut src)
    }

    pub fn from_buf<B>(src: &mut B) -> Option<Self>
    where
        B: Buf,
    {
        let length = src.remaining(); // Needs 1 + [0 | 2 | 4 | 8] octets
        if length > 0 {
            let value = match src.get_u8() {
                x @ 0..=252 => u64::from(x),
                253 if length > 2 => u64::from(src.get_u16()),
                254 if length > 4 => u64::from(src.get_u32()),
                255 if length > 8 => src.get_u64(),
                _ => return None,
            };
            Some(Self::from_u64(value))
        } else {
            None
        }
    }

    pub fn peek(src: &[u8]) -> Option<Self> {
        let mut src = std::io::Cursor::new(src);
        Self::from_buf(&mut src)
    }

    pub fn encode(&self, dst: &mut BytesMut) {
        dst.extend(&self.bytes);
    }

    pub fn decode(src: &mut BytesMut) -> Option<Self> {
        Self::from_buf(src)
    }
}

impl ops::Deref for VarNumber {
    type Target = u64;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl ops::Add for VarNumber {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::from_u64(self.to_u64() + rhs.to_u64())
    }
}

impl ops::Add<u64> for VarNumber {
    type Output = Self;

    fn add(self, rhs: u64) -> Self::Output {
        Self::from_u64(self.to_u64() + rhs)
    }
}

impl From<u8> for VarNumber {
    #[inline]
    fn from(u: u8) -> Self {
        Self::from_u64(u64::from(u))
    }
}

impl From<u16> for VarNumber {
    #[inline]
    fn from(u: u16) -> Self {
        Self::from_u64(u64::from(u))
    }
}

impl From<u32> for VarNumber {
    #[inline]
    fn from(u: u32) -> Self {
        Self::from_u64(u64::from(u))
    }
}

impl From<u64> for VarNumber {
    #[inline]
    fn from(u: u64) -> Self {
        Self::from_u64(u)
    }
}

impl From<usize> for VarNumber {
    #[inline]
    fn from(u: usize) -> Self {
        Self::from_u64(u as u64)
    }
}

impl From<VarNumber> for u64 {
    fn from(value: VarNumber) -> Self {
        value.to_u64()
    }
}

impl fmt::Display for VarNumber {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(fmt, "{}", self.to_u64())
    }
}

impl From<VarNumber> for Bytes {
    fn from(v: VarNumber) -> Self {
        v.bytes
    }
}

impl cmp::PartialEq for VarNumber {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}

impl cmp::PartialEq<u64> for VarNumber {
    fn eq(&self, other: &u64) -> bool {
        self.value == *other
    }
}

// impl cmp::PartialEq<usize> for VarNumber {
//     fn eq(&self, other: &usize) -> bool {
//         self.value == (*other as u64)
//     }
// }

impl cmp::Eq for VarNumber {}

impl cmp::PartialOrd for VarNumber {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl cmp::PartialOrd<u64> for VarNumber {
    fn partial_cmp(&self, other: &u64) -> Option<cmp::Ordering> {
        self.value.partial_cmp(other)
    }
}

impl cmp::Ord for VarNumber {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.value.cmp(&other.value)
    }
}

impl hash::Hash for VarNumber {
    fn hash<H: hash::Hasher>(&self, state: &mut H) {
        self.value.hash(state);
    }
}

impl Extend<VarNumber> for BytesMut {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = VarNumber>,
    {
        for item in iter {
            self.extend_from_slice(&item.bytes)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_conversion() {
        let vn: VarNumber = 2_u8.into();
        assert_eq!(vn, VarNumber::from_u64(2));

        let vn: VarNumber = 255_u8.into();
        assert_eq!(vn, VarNumber::from_u64(255));

        let vn: VarNumber = 55678_u16.into();
        assert_eq!(vn, VarNumber::from_u64(55678));

        let vn: VarNumber = 345_345_344_u32.into();
        assert_eq!(vn, VarNumber::from_u64(345_345_344));

        let vn: VarNumber = 87_234_298_734_844_u64.into();
        assert_eq!(vn, VarNumber::from_u64(87_234_298_734_844));
    }

    #[test]
    fn one_byte() {
        let bytes: Bytes = VarNumber::from_u64(0).into();
        assert_eq!(bytes, vec![0]);

        let bytes: Bytes = VarNumber::from_u64(128).into();
        assert_eq!(bytes, vec![128]);

        let bytes: Bytes = VarNumber::from_u64(252).into();
        assert_eq!(bytes, vec![252]);
    }

    #[test]
    fn two_bytes65530() {
        let bytes: Bytes = VarNumber::from_u64(65530).into();
        assert_eq!(bytes, vec![253, 255, 250]);
    }

    #[test]
    fn four_bytes_0xff_34_56_da() {
        let bytes: Bytes = VarNumber::from_u64(0xff_34_56_da).into();
        assert_eq!(bytes, vec![254, 0xff, 0x34, 0x56, 0xda]);
    }

    #[test]
    fn eight_bytes_0x12_34_56_78_9a_bc_de_f0() {
        let bytes: Bytes = VarNumber::from_u64(0x12_34_56_78_9a_bc_de_f0).into();
        assert_eq!(
            bytes,
            vec![255, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]
        );
    }

    #[test]
    fn varnumber_00() {
        let bytes = VarNumber::from_slice(&[0_u8]).unwrap();
        assert_eq!(VarNumber::from_u64(0), bytes);
    }

    #[test]
    fn varnumber_128() {
        let bytes = VarNumber::from_slice(&[128_u8]).unwrap();
        assert_eq!(VarNumber::from_u64(128), bytes);
    }

    #[test]
    fn varnumber_252() {
        let bytes = VarNumber::from_slice(&[252u8]).unwrap();
        assert_eq!(VarNumber::from_u64(252), bytes);
    }

    #[test]
    fn varnumber_65530() {
        let bytes = VarNumber::from_slice(&[253u8, 255u8, 250u8]).unwrap();
        assert_eq!(VarNumber::from_u64(65530), bytes);
    }

    #[test]
    fn varnumber_1234567890() {
        let bytes = VarNumber::from_slice(&[254, 0x49, 0x96, 0x02, 0xd2]).unwrap();
        assert_eq!(VarNumber::from_u64(1_234_567_890), bytes);
    }

    #[test]
    fn varnumber_12345678901234567890() {
        let bytes = VarNumber::from_slice(&[255, 171, 84, 169, 140, 235, 31, 10, 210]).unwrap();
        assert_eq!(VarNumber::from_u64(12_345_678_901_234_567_890), bytes);
    }

    #[test]
    fn decode_empty_buf() {
        let mut src = BytesMut::new();
        assert!(VarNumber::from_buf(&mut src).is_none());
    }

    #[test]
    fn decode_empty_slice() {
        let src = [];
        assert!(VarNumber::from_slice(&src).is_none());
    }

    #[test]
    fn decode_00() {
        let n = VarNumber::from_slice(&[0]).unwrap();
        assert_eq!(n, 0_u64);
    }

    #[test]
    fn decode_01() {
        let n = VarNumber::from_slice(&[1]).unwrap();
        assert_eq!(n, 1_u64);
    }

    #[test]
    fn decode_128() {
        let n = VarNumber::from_slice(&[128]).unwrap();
        assert_eq!(n, 128_u64);
    }

    #[test]
    fn decode_252() {
        let n = VarNumber::from_slice(&[252]).unwrap();
        assert_eq!(n, 252_u64);
    }

    #[test]
    fn decode_253_invalid() {
        assert!(VarNumber::from_slice(&[253]).is_none());
    }

    #[test]
    fn decode_253() {
        let n = VarNumber::from_slice(&[253, 0, 253]).unwrap();
        assert_eq!(n, 253_u64);
    }

    #[test]
    fn decode_65530() {
        let n = VarNumber::from_slice(&[253u8, 255u8, 250u8]).unwrap();
        assert_eq!(n, 65530_u64)
    }

    #[test]
    fn decode_1234567890() {
        let n = VarNumber::from_slice(&[254, 0x49, 0x96, 0x02, 0xd2]).unwrap();
        assert_eq!(n, 1_234_567_890_u64);
    }

    #[test]
    fn decode_12345678901234567890() {
        let n = VarNumber::from_slice(&[255, 171, 84, 169, 140, 235, 31, 10, 210]).unwrap();
        assert_eq!(n, 12_345_678_901_234_567_890_u64);
    }

    #[test]
    fn decode_and_advance() {
        let mut src = Bytes::from_static(&[253, 255, 250, 0]);
        assert_eq!(src.remaining(), 4);
        let n = VarNumber::from_buf(&mut src).unwrap();
        assert_eq!(n, 65530_u64);
        assert_eq!(src.remaining(), 1);
    }

    #[test]
    fn peek_over() {
        let src = Bytes::from_static(&[253, 255, 250, 233]);
        assert_eq!(src.remaining(), 4);
        let n = VarNumber::peek(&src).unwrap();
        assert_eq!(n, 65530_u64);
        assert_eq!(src.remaining(), 4);
    }

    #[test]
    fn peek_exact() {
        let src = Bytes::from_static(&[253, 255, 251]);
        assert_eq!(src.remaining(), 3);
        let n = VarNumber::peek(&src).unwrap();
        assert_eq!(n, 65531_u64);
        assert_eq!(src.remaining(), 3);
    }

    #[test]
    fn peek_under() {
        let src = Bytes::from_static(&[253, 255]);
        assert_eq!(src.remaining(), 2);
        assert!(dbg!(VarNumber::peek(&src)).is_none());
        assert_eq!(src.remaining(), 2);
    }
}
