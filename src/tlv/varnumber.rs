use std::convert::TryFrom;
use std::fmt;
use std::mem;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct VarNumber(u64);

impl fmt::Display for VarNumber {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "{}", self.0)
    }
}

impl From<VarNumber> for Vec<u8> {
    fn from(v: VarNumber) -> Self {
        match v.0 {
            x @ 0...252 => vec![x as u8],
            x @ 253...0xFFFF => {
                let mut result = Vec::with_capacity(3);
                result.push(253u8);
                let bytes: [u8; 2] = unsafe { mem::transmute(u16::to_be(x as u16)) };
                result.extend_from_slice(&bytes);
                result
            }
            x @ 0x1_0000...0xFFFF_FFFF => {
                let mut result = Vec::with_capacity(5);
                result.push(254u8);
                let bytes: [u8; 4] = unsafe { mem::transmute(u32::to_be(x as u32)) };
                result.extend_from_slice(&bytes);
                result
            }
            x @ 0x1_0000_0000...0xFFFF_FFFF_FFFF_FFFF => {
                let mut result = Vec::with_capacity(9);
                result.push(255u8);
                let bytes: [u8; 8] = unsafe { mem::transmute(u64::to_be(x)) };
                result.extend_from_slice(&bytes);
                result
            }
            _ => unreachable!(),
        }
    }
}

impl<'a> TryFrom<&'a [u8; 1]> for VarNumber {
    type Err = ();
    fn try_from(s: &'a [u8; 1]) -> Result<Self, ()> {
        match s[0] {
            253...255 => Err(()),
            x => Ok(VarNumber(x as u64)),
        }
    }
}

impl<'a> TryFrom<&'a [u8; 3]> for VarNumber {
    type Err = ();
    fn try_from(s: &'a [u8; 3]) -> Result<Self, ()> {
        if s[0] == 253 {
            Ok(VarNumber(((s[1] as u64) << 8) + s[2] as u64))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a [u8; 5]> for VarNumber {
    type Err = ();
    fn try_from(s: &'a [u8; 5]) -> Result<Self, ()> {
        if s[0] == 254 {
            Ok(VarNumber(((s[1] as u64) << 24) + ((s[2] as u64) << 16) + ((s[3] as u64) << 8) +
                         (s[4] as u64)))
        } else {
            Err(())
        }
    }
}

impl<'a> TryFrom<&'a [u8; 9]> for VarNumber {
    type Err = ();
    fn try_from(s: &'a [u8; 9]) -> Result<Self, ()> {
        if s[0] == 255 {
            Ok(VarNumber(((s[1] as u64) << 56) +
                         ((s[2] as u64) << 48) +
                         ((s[3] as u64) << 40) +
                         ((s[4] as u64) << 32) +
                         ((s[5] as u64) << 24) +
                         ((s[6] as u64) << 16) +
                         ((s[7] as u64) << 8) +
                         (s[8] as u64)))
        } else {
            Err(())
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
        assert_eq!(bytes, vec![254, 0xff, 0x34, 0x56, 0xda]);
    }

    #[test]
    fn eight_bytes_0x12_34_56_78_9a_bc_de_f0() {
        let bytes: Vec<u8> = VarNumber(0x12_34_56_78_9a_bc_de_f0).into();
        assert_eq!(bytes, vec![255, 0x12, 0x34, 0x56, 0x78, 0x9a, 0xbc, 0xde, 0xf0]);
    }

    #[test]
    fn varnumber_00() {
        let val = VarNumber::try_from(&[0u8]).unwrap();
        assert_eq!(VarNumber(0), val);
    }

    #[test]
    fn varnumber_128() {
        let val = VarNumber::try_from(&[128u8]).unwrap();
        assert_eq!(VarNumber(128), val);
    }

    #[test]
    fn varnumber_252() {
        let val = VarNumber::try_from(&[252u8]).unwrap();
        assert_eq!(VarNumber(252), val);
    }

    #[test]
    fn varnumber_65530() {
        let val = VarNumber::try_from(&[253u8, 255u8, 250u8]).unwrap();
        assert_eq!(VarNumber(65530), val);
    }

    #[test]
    fn varnumber_1234567890() {
        let val = VarNumber::try_from(&[254, 0x49, 0x96, 0x02, 0xd2]).unwrap();
        assert_eq!(VarNumber(1234567890), val);
    }

    #[test]
    fn varnumber_12345678901234567890() {
        let val = VarNumber::try_from(&[255, 171, 84, 169, 140, 235, 31, 10, 210]).unwrap();
        assert_eq!(VarNumber(12345678901234567890), val);
    }
}
