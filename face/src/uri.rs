use super::*;

pub use addr::Addr;
pub use addr::Internal;
pub use addr::Tcp;
pub use addr::Udp;
pub use addr::Unix;

mod addr;

pub const URI_DELIMITER: &str = "://";

tlv::utf8_string!(Uri => tlv::Type::Uri);
tlv::utf8_string!(LocalUri => tlv::Type::LocalUri);

impl Uri {
    pub async fn to_addr(&self) -> io::Result<Addr> {
        Addr::from_uri(self).await
    }
}

impl LocalUri {
    pub async fn to_addr(&self) -> io::Result<Addr> {
        Addr::from_uri(self).await
    }
}
