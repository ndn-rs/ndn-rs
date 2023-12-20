use std::error::Error as StdError;
use std::fmt;
use std::io;
use std::str;

use bytes::{Buf, BufMut, Bytes, BytesMut};

pub use ndn_tlv_derive::Tlv;
pub use ndn_varnumber::VarNumber;

pub use codec::TlvCodec;
pub use tlv::Tlv;

mod codec;
mod impls;
mod tlv;
mod types;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Type(u64);

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
