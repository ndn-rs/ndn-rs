use super::*;

non_negative_number!(ContentType => Type::ContentType);

impl ContentType {
    pub const BLOB: Self = Self(NonNegativeNumber(0));
    pub const LINK: Self = Self(NonNegativeNumber(1));
    pub const KEY: Self = Self(NonNegativeNumber(2));
    pub const NACK: Self = Self(NonNegativeNumber(3));
}
