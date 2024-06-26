use super::*;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Type mismatch: expecting {0}, found {1:?}")]
    TypeMismatch(Type, Generic),

    #[error("Length mismatch: expected {expected}, found {found}")]
    LengthMismatch { expected: usize, found: usize },

    #[error("Invalid (corrupted?) Data: ({0})")]
    InvalidData(String),

    #[error("IO Error: {0}")]
    Io(#[from] io::Error),

    #[error("{0}")]
    Other(String),
}

impl DecodeError {
    pub fn r#type(r#type: Type, generic: Generic) -> Self {
        Self::TypeMismatch(r#type, generic)
    }

    pub fn invalid(reason: impl Into<String>) -> Self {
        let reason = reason.into();
        Self::InvalidData(reason)
    }

    pub fn other(msg: impl Into<String>) -> Self {
        let msg = msg.into();
        Self::Other(msg)
    }

    pub const fn length_mismatch(expected: usize, found: usize) -> Self {
        Self::LengthMismatch { expected, found }
    }
}

impl From<DecodeError> for io::Error {
    fn from(err: DecodeError) -> Self {
        match err {
            DecodeError::Io(err) => err,
            other => Self::other(other),
        }
    }
}
