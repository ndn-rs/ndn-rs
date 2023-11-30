use std::path::{Path, PathBuf};

use super::*;

#[derive(Clone, Debug)]
pub struct Unix {
    pub path: PathBuf,
}

impl Unix {
    pub const PREFIX: &'static str = "unix";

    pub async fn from_uri(unix: &str, addr: &str) -> io::Result<Self> {
        if unix == Self::PREFIX {
            Ok(Self::new(addr))
        } else {
            Err(io::Error::other("Invalid prefix for unix face"))
        }
    }

    fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref().to_path_buf();
        Self { path }
    }
}
