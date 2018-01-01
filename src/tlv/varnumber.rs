use std::fmt;
use std::ops;

use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut, IntoBuf};
use bytes::buf::FromBuf;

#[derive(Debug, Clone, PartialEq, Hash)]
pub struct VarNumber {
    inner: Bytes,
    value: u64,
}

impl VarNumber {
    pub fn length(&self) -> usize {
        self.inner.len()
    }

    fn from_u64(u: u64) -> Self {
        let bytes = match u {
            x @ 0...252 => {
                let mut bytes = BytesMut::with_capacity(1);
                bytes.put_u8(x as u8);
                bytes
            }
            x @ 253...0xFFFF => {
                let mut bytes = BytesMut::with_capacity(3);
                bytes.put_u8(253);
                bytes.put_u16::<BigEndian>(x as u16);
                bytes
            }
            x @ 0x1_0000...0xFFFF_FFFF => {
                let mut bytes = BytesMut::with_capacity(5);
                bytes.put_u8(254);
                bytes.put_u32::<BigEndian>(x as u32);
                bytes
            }
            x @ 0x1_0000_0000...0xFFFF_FFFF_FFFF_FFFF => {
                let mut bytes = BytesMut::with_capacity(9);
                bytes.put_u8(255);
                bytes.put_u64::<BigEndian>(x);
                bytes
            }
            _ => unreachable!(),
        };
        Self {
            inner: bytes.freeze(),
            value: u,
        }
    }

    fn as_u64(&self) -> u64 {
        self.value
    }
}

impl ops::Add for VarNumber {
    type Output = VarNumber;

    fn add(self, rhs: VarNumber) -> Self::Output {
        VarNumber::from_u64(self.as_u64() + rhs.as_u64())
    }
}

impl ops::Add<u64> for VarNumber {
    type Output = VarNumber;

    fn add(self, rhs: u64) -> Self::Output {
        VarNumber::from_u64(self.as_u64() + rhs)
    }
}

impl From<u8> for VarNumber {
    #[inline]
    fn from(u: u8) -> Self {
        VarNumber::from_u64(u64::from(u))
    }
}

impl From<u16> for VarNumber {
    #[inline]
    fn from(u: u16) -> Self {
        VarNumber::from_u64(u64::from(u))
    }
}

impl From<u32> for VarNumber {
    #[inline]
    fn from(u: u32) -> Self {
        VarNumber::from_u64(u64::from(u))
    }
}

impl From<u64> for VarNumber {
    #[inline]
    fn from(u: u64) -> Self {
        VarNumber::from_u64(u)
    }
}

impl From<usize> for VarNumber {
    #[inline]
    fn from(u: usize) -> Self {
        VarNumber::from_u64(u as u64)
    }
}

impl fmt::Display for VarNumber {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.as_u64())
    }
}

impl From<VarNumber> for Bytes {
    fn from(v: VarNumber) -> Self {
        v.inner.clone()
    }
}

impl FromBuf for VarNumber {
    fn from_buf<B>(buf: B) -> Self
    where
        B: IntoBuf,
    {
        let mut buf = buf.into_buf();
        let n = match buf.get_u8() {
            x @ 0...252 => u64::from(x),
            253 => u64::from(buf.get_u16::<BigEndian>()),
            254 => u64::from(buf.get_u32::<BigEndian>()),
            255 => buf.get_u64::<BigEndian>(),
            _ => unreachable!(),
        };
        VarNumber::from_u64(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_conversion() {
        let vn: VarNumber = 2u8.into();
        assert_eq!(vn, VarNumber::from_u64(2));

        let vn: VarNumber = 255u8.into();
        assert_eq!(vn, VarNumber::from_u64(255));

        let vn: VarNumber = 55678u16.into();
        assert_eq!(vn, VarNumber::from_u64(55678));

        let vn: VarNumber = 345345344u32.into();
        assert_eq!(vn, VarNumber::from_u64(345345344));

        let vn: VarNumber = 87234298734844u64.into();
        assert_eq!(vn, VarNumber::from_u64(87234298734844));
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
        let bytes = Bytes::from_static(&[0u8]).into_buf();
        assert_eq!(VarNumber::from_u64(0), bytes.collect());
    }

    #[test]
    fn varnumber_128() {
        let bytes = Bytes::from_static(&[128u8]).into_buf().into_buf();
        assert_eq!(VarNumber::from_u64(128), bytes.collect());
    }

    #[test]
    fn varnumber_252() {
        let bytes = Bytes::from_static(&[252u8]).into_buf();
        assert_eq!(VarNumber::from_u64(252), bytes.collect());
    }

    #[test]
    fn varnumber_65530() {
        let bytes = Bytes::from_static(&[253u8, 255u8, 250u8]).into_buf();
        assert_eq!(VarNumber::from_u64(65530), bytes.collect());
    }

    #[test]
    fn varnumber_1234567890() {
        let bytes = Bytes::from_static(&[254, 0x49, 0x96, 0x02, 0xd2]).into_buf();
        assert_eq!(VarNumber::from_u64(1234567890), bytes.collect());
    }

    #[test]
    fn varnumber_12345678901234567890() {
        let bytes = Bytes::from_static(&[255, 171, 84, 169, 140, 235, 31, 10, 210]).into_buf();
        assert_eq!(VarNumber::from_u64(12345678901234567890), bytes.collect());
    }
}
