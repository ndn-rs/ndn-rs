use std::fmt;
use std::str;

use bytes::{Buf, Bytes, BytesMut};
use generic_array::typenum::{U32, U4};
use generic_array::GenericArray;
use thiserror::Error;

use ndn_varnumber::VarNumber;

pub use application::ApplicationParameters;
pub use canbeprefix::CanBePrefix;
pub use content::Content;
pub use contenttype::ContentType;
pub use data::Data;
pub use error::DecodeError;
pub use fresh::FreshnessPeriod;
pub use fresh::MustBeFresh;
pub use generic::Generic;
pub use hint::ForwardingHint;
pub use hoplimit::HopLimit;
pub use interest::Interest;
pub use interest::InterestLifetime;
pub use metainfo::MetaInfo;
pub use name::ByteOffsetNameComponent;
pub use name::FinalBlockId;
pub use name::GenericNameComponent;
pub use name::ImplicitSha256DigestComponent;
pub use name::KeywordNameComponent;
pub use name::Name;
pub use name::NameComponent;
pub use name::NameError;
pub use name::OtherTypeComponent;
pub use name::ParametersSha256DigestComponent;
pub use name::SegmentNameComponent;
pub use name::SequenceNumNameComponent;
pub use name::TimestampNameComponent;
pub use name::VersionNameComponent;
pub use nonce::Nonce;
pub use number::NonNegativeNumber;
pub use signature::DataSignature;
pub use signature::InterestSignature;
pub use signature::InterestSignatureInfo;
pub use signature::InterestSignatureValue;
pub use signature::SignatureInfo;
pub use signature::SignatureValue;

mod application;
mod canbeprefix;
mod content;
mod contenttype;
mod data;
mod fresh;
mod generic;
mod hint;
mod hoplimit;
mod interest;
mod metainfo;
mod name;
mod nonce;
mod number;
mod octets;
mod signature;
mod string;

mod error;
mod impls;
mod types;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type(u64);

pub trait Tlv: fmt::Debug {
    // /// Each TLV type has its assigned TLV-TYPE number defined as a constant of type u64
    // const TYPE: Type;

    /// Report this TLV-TYPE as `Type`
    fn r#type(&self) -> Type;

    /// report this TLV-TYPE as a `VarNumber`
    fn type_as_varnumber(&self) -> VarNumber {
        self.r#type().to_varnumber()
    }

    /// Report TLV-LENGTH as a `VarNumber`
    fn length(&self) -> VarNumber {
        self.payload_size().into()
    }

    /// Report TLV-VALUE as `Bytes` buffer (if value is present)
    fn value(&self) -> Option<Bytes>;

    /// Report the total size of the packet or TLV element, including the TLV-TYPE and TLV-LENGTH
    fn size(&self) -> usize {
        self.payload_size() + self.type_as_varnumber().len() + self.length().len()
    }

    /// Report the size of the payload if any
    fn payload_size(&self) -> usize;

    /// Convert this TLV to `Bytes`
    fn bytes(&self) -> Bytes {
        let mut bytes = BytesMut::new();
        self.write(&mut bytes);
        bytes.freeze()
    }

    /// Write this TLV to `BytesMut`
    fn write(&self, dst: &mut BytesMut) {
        let r#type = self.type_as_varnumber().bytes();
        let length = self.length().bytes();
        let value = self.value().unwrap_or_default();
        let additional = r#type.len() + length.len() + value.len();
        dst.reserve(additional);
        dst.extend([r#type, length, value]);
    }
}

impl<T: Tlv> Tlv for Option<T> {
    fn r#type(&self) -> Type {
        self.as_ref()
            .expect("Cannot call .r#type() on None")
            .r#type()
    }

    fn value(&self) -> Option<Bytes> {
        self.as_ref().and_then(|t| t.value())
    }

    fn size(&self) -> usize {
        self.as_ref().map(|t| t.size()).unwrap_or_default()
    }

    fn payload_size(&self) -> usize {
        self.as_ref().map(|t| t.payload_size()).unwrap_or_default()
    }

    fn bytes(&self) -> Bytes {
        self.as_ref().map(|t| t.bytes()).unwrap_or_default()
    }
}

pub fn collect_to_bytes<I, O>(items: I) -> Option<Bytes>
where
    I: IntoIterator<Item = O>,
    O: Into<Option<Bytes>>,
{
    let items = items.into_iter().filter_map(|item| item.into());
    let mut bytes = BytesMut::new();
    bytes.extend(items);
    if bytes.is_empty() {
        None
    } else {
        Some(bytes.freeze())
    }
}

pub fn display_option<T>(item: &Option<T>, f: &mut fmt::Formatter<'_>) -> fmt::Result
where
    T: fmt::Display,
{
    use fmt::Display;

    item.as_ref()
        .map_or(Ok(()), |item| format_args!(" {item:#}").fmt(f))
}
