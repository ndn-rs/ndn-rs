#[derive(Debug, thiserror::Error)]
#[error("Invalid FaceUri '{uri}': - {reason}")]
pub struct InvalidFaceUri {
    uri: String,
    reason: String,
}

impl InvalidFaceUri {
    pub(crate) fn new(uri: &str, reason: impl ToString) -> Self {
        let uri = uri.to_string();
        let reason = reason.to_string();
        Self { uri, reason }
    }
}
