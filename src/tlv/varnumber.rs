use std::fmt;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarNumber(u64);

impl fmt::Display for VarNumber {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl Into<Vec<u8>> for VarNumber {
    fn into(self) -> Vec<u8> {
        match self.0 {
            x @ 0 ... 252 => vec![x as u8],
            x @ 253 ... 0xFFFF => {
                let mut result = Vec::with_capacity(3);
                result.push(253u8);
                let bytes: [u8; 2] = unsafe { mem::transmute(u16::to_be(x as u16)) };
                result.extend_from_slice(&bytes);
                result
            },
            x @ 0x1_0000 ... 0xFFFF_FFFF => {
                let mut result = Vec::with_capacity(5);
                result.push(254u8);
                let bytes: [u8; 4] = unsafe { mem::transmute(u32::to_be(x as u32)) };
                result.extend_from_slice(&bytes);
                result
            },
            x @ 0x1_0000_0000 ... 0xFFFF_FFFF_FFFF_FFFF => {
                let mut result = Vec::with_capacity(9);
                result.push(255u8);
                let bytes: [u8; 8] = unsafe { mem::transmute(u64::to_be(x)) };
                result.extend_from_slice(&bytes);
                result
            },
            _ => unreachable!(),
        }
    }
}

impl<'a> From<&'a [u8]> for VarNumber {
    fn from(s: &'a [u8]) -> Self {
        match s[0] {
            255 => VarNumber(0),
            254 => VarNumber(0),
            253 => VarNumber(0),
            x @ _ => VarNumber(x as u64),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_byte0() {
        let bytes: Vec<u8> = VarNumber(0).into();
        assert_eq!(bytes, vec![0u8]);
    }

    #[test]
    fn one_byte128() {
        let bytes: Vec<u8> = VarNumber(128).into();
        assert_eq!(bytes, vec![128u8]);
    }

    #[test]
    fn one_byte252() {
        let bytes: Vec<u8> = VarNumber(252).into();
        assert_eq!(bytes, vec![252u8]);
    }

    #[test]
    fn two_bytes65530() {
        let bytes: Vec<u8> = VarNumber(65530).into();
        assert_eq!(bytes, vec![253u8, 255u8, 250u8]);
    }

    #[test]
    fn four_bytes_0xff_34_56_da() {
        let bytes: Vec<u8> = VarNumber(0xff_34_56_da).into();
        assert_eq!(bytes, vec![254u8, 0xffu8, 0x34u8, 0x56u8, 0xdau8]);
    }

    // #[test]
    // fn varnumber_00() {
    //     let val = VarNumber::from(&[0u8]);
    //     assert_eq!(VarNumber(0), val);
    // }
}
