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
non_negative_number!(SignatureType => Type::SignatureType; skip_display);
non_negative_number!(SignatureTime => Type::SignatureTime);
non_negative_number!(SignatureSeqNum => Type::SignatureSeqNum);

#[allow(non_upper_case_globals)]
impl SignatureType {
    pub const DigestSha256: Self = Self(NonNegativeNumber(0));
    pub const SignatureSha256WithRsa: Self = Self(NonNegativeNumber(1));
    pub const SignatureSha256WithEcdsa: Self = Self(NonNegativeNumber(3));
    pub const SignatureHmacWithSha256: Self = Self(NonNegativeNumber(4));
    pub const SignatureEd25519: Self = Self(NonNegativeNumber(5));

    pub fn needs_key_locator(&self) -> bool {
        match self {
            &Self::DigestSha256 => false,
            &Self::SignatureSha256WithRsa => true,
            &Self::SignatureSha256WithEcdsa => true,
            &Self::SignatureHmacWithSha256 => true,
            &Self::SignatureEd25519 => true,
            _other => false,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            &Self::DigestSha256 => "sha256",
            &Self::SignatureSha256WithRsa => "sha256+rsa",
            &Self::SignatureSha256WithEcdsa => "sha256+ecdsa",
            &Self::SignatureHmacWithSha256 => "hmac+sha256",
            &Self::SignatureEd25519 => "ed25519",
            _other => "unknown",
        }
    }
}

impl fmt::Display for SignatureType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.as_str().fmt(f)
    }
}
