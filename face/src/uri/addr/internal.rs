use super::*;

#[derive(Debug)]
pub struct Internal;

impl Internal {
    pub const PREFIX: &'static str = "internal";

    pub async fn from_uri(prefix: &str, addr: &str) -> io::Result<Self> {
        if prefix != Self::PREFIX {
            Err(io::Error::other("Invalid prefix for internal face"))
        } else if !addr.is_empty() {
            Err(io::Error::other("Invalid addr for internal face"))
        } else {
            Ok(Self)
        }
    }

    pub(super) fn any() -> Self {
        Self
    }
}
