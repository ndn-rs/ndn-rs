use super::*;

#[derive(Debug, Error)]
pub enum DecodeError {
    #[error("Type mismatch")]
    TypeMismatch(Generic),

    #[error("Length mismatch")]
    LengthMismatch(Generic),

    #[error("Invalid Data")]
    InvalidData,
}
