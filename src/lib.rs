//! Named Data Networking (aka Internet 2.0)
//!
#![doc(html_root_url = "https://docs.rs/ndn/0.0.1")]
#![cfg_attr(all(feature = "cargo-clippy", feature = "pedantic"), warn(clippy_pedantic))]

extern crate bytes;

pub mod error;
pub mod tlv;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {}
}
