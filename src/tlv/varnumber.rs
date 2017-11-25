use std::fmt;
use bytes::{BigEndian, Buf, BufMut, Bytes, BytesMut, IntoBuf};

#[derive(Debug, Copy, Clone, PartialEq, Hash)]
pub struct VarNumber(u64);

impl fmt::Display for VarNumber {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl From<u64> for VarNumber {
    fn from(u: u64) -> Self {
        VarNumber(u)
    }
}

impl From<VarNumber> for Bytes {
    fn from(v: VarNumber) -> Self {
        let bytes = match v.0 {
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
        bytes.freeze()
    }
}

impl<B: IntoBuf> From<B> for VarNumber {
    fn from(buf: B) -> Self {
        let mut buf = buf.into_buf();
        let n = match buf.get_u8() {
            x @ 0...252 => u64::from(x),
            253 => u64::from(buf.get_u16::<BigEndian>()),
            254 => u64::from(buf.get_u32::<BigEndian>()),
            255 => buf.get_u64::<BigEndian>(),
            _ => unreachable!(),
        };
        VarNumber(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_byte0() {
        let bytes: Bytes = VarNumber(0).into();
        assert_eq!(bytes, vec![0]);
    }

    #[test]
    fn one_byte128() {
        let bytes: Bytes = VarNumber(128).into();
        assert_eq!(bytes, vec![128]);
    }

    #[test]
    fn one_byte252() {
        let bytes: Bytes = VarNumber(252).into();
        assert_eq!(bytes, vec![252]);
    }

    #[test]
    fn two_bytes65530() {
        let bytes: Bytes = VarNumber(65530).into();
        assert_eq!(bytes, vec![253, 255, 250]);
    }

    #[test]
    fn four_bytes_0xff_34_56_da() {
        let bytes: Bytes = VarNumber(0xff_34_56_da).into();
        assert_eq!(bytes, vec![254, 0xff, 0x34, 0x56, 0xda]);
    }

    #[test]
    fn eight_bytes_0x12_34_56_78_9a_bc_de_f0() {
        let bytes: Bytes = VarNumber(0x12_34_56_78_9a_bc_de_f0).into();
        assert_eq!(
            bytes,
            vec![255, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]
        );
    }

    #[test]
    fn varnumber_00() {
        let bytes = Bytes::from_static(&[0u8]);
        assert_eq!(VarNumber(0), bytes.into());
    }

    #[test]
    fn varnumber_128() {
        let bytes = Bytes::from_static(&[128u8]);
        assert_eq!(VarNumber(128), bytes.into());
    }

    #[test]
    fn varnumber_252() {
        let bytes = Bytes::from_static(&[252u8]);
        assert_eq!(VarNumber(252), bytes.into());
    }

    #[test]
    fn varnumber_65530() {
        let bytes = Bytes::from_static(&[253u8, 255u8, 250u8]);
        assert_eq!(VarNumber(65530), bytes.into());
    }

    #[test]
    fn varnumber_1234567890() {
        let bytes = Bytes::from_static(&[254, 0x49, 0x96, 0x02, 0xd2]);
        assert_eq!(VarNumber(1234567890), bytes.into());
    }

    #[test]
    fn varnumber_12345678901234567890() {
        let bytes = Bytes::from_static(&[255, 171, 84, 169, 140, 235, 31, 10, 210]);
        assert_eq!(VarNumber(12345678901234567890), bytes.into());
    }
}
