use super::*;

pub use data::DataSignature;
pub use data::SignatureInfo;
pub use data::SignatureValue;
pub use interest::InterestSignature;
pub use interest::InterestSignatureInfo;
pub use interest::InterestSignatureValue;
pub use keylocator::KeyDigest;
pub use keylocator::KeyLocator;

mod data;
mod interest;
mod keylocator;

octets!(SignatureNonce => Type::SignatureNonce);
non_negative_number!(SignatureType => Type::SignatureType);
non_negative_number!(SignatureTime => Type::SignatureTime);
non_negative_number!(SignatureSeqNum => Type::SignatureSeqNum);

#[allow(non_upper_case_globals)]
impl SignatureType {
    pub const DigestSha256: Self = Self(NonNegativeNumber(0));
    pub const SignatureSha256WithRsa: Self = Self(NonNegativeNumber(1));
    pub const SignatureSha256WithEcdsa: Self = Self(NonNegativeNumber(3));
    pub const SignatureHmacWithSha256: Self = Self(NonNegativeNumber(4));
    pub const SignatureEd25519: Self = Self(NonNegativeNumber(5));

    // pub fn need_key_locator(&self) -> bool {
    //     match self {
    //         Self::DigestSha256 => false,
    //         Self::SignatureSha256WithRsa => true,
    //         Self::SignatureSha256WithEcdsa => true,
    //         Self::SignatureHmacWithSha256 => true,
    //         Self::SignatureEd25519 => true,
    //     }
    // }
}
