use std::fmt;

use ndn_varnumber::VarNumber;

pub use de::from_bytes;

pub mod de;
pub mod ser;

#[derive(Debug, thiserror::Error)]
#[error("GPF")]
pub struct Error {
    reason: String,
}

impl Error {
    fn trailing_bytes() -> Self {
        let reason = String::from("Trailing bytes");
        Self { reason }
    }

    fn not_enough_bytes() -> Self {
        let reason = String::from("Not enough bytes");
        Self { reason }
    }
}

impl serde::ser::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        let reason = msg.to_string();
        Self { reason }
    }
}

impl serde::de::Error for Error {
    fn custom<T>(msg: T) -> Self
    where
        T: fmt::Display,
    {
        let reason = msg.to_string();
        Self { reason }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use serde::{Deserialize, Serialize};

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Version(u64);

    #[test]
    fn u64_0() {
        let v = from_bytes::<Version>(&[0]).unwrap();
        assert_eq!(v, Version(0));
    }

    #[test]
    fn u64_1() {
        let v = from_bytes::<Version>(&[1]).unwrap();
        assert_eq!(v, Version(1));
    }

    #[test]
    fn u64_128() {
        let v = from_bytes::<Version>(&[128]).unwrap();
        assert_eq!(v, Version(128));
    }

    #[test]
    fn u64_252() {
        let v = from_bytes::<Version>(&[252]).unwrap();
        assert_eq!(v, Version(252));
    }

    #[test]
    fn u64_253() {
        let v = from_bytes::<Version>(&[253, 0, 253]).unwrap();
        assert_eq!(v, Version(253));
    }

    #[test]
    fn u64_253_invalid() {
        from_bytes::<Version>(&[253]).unwrap_err();
    }

    #[test]
    fn u64_65530() {
        let v = from_bytes::<Version>(&[253, 255, 250]).unwrap();
        assert_eq!(v, Version(65530));
    }

    #[test]
    fn u64_1234567890() {
        let v = from_bytes::<Version>(&[254, 0x49, 0x96, 0x02, 0xd2]).unwrap();
        assert_eq!(v, Version(1234567890));
    }

    #[test]
    fn u64_12345678901234567890() {
        let v = from_bytes::<Version>(&[255, 171, 84, 169, 140, 235, 31, 10, 210]).unwrap();
        assert_eq!(v, Version(12345678901234567890));
    }
}
