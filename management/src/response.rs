use super::*;

#[derive(Debug)]
pub struct ControlResponse {
    pub status_code: StatusCode,
    pub status_text: StatusText,
    pub body: Vec<tlv::Generic>,
}

impl ControlResponse {
    pub fn incorrect_control_parameters(reason: impl ToString) -> Self {
        let status_code = StatusCode::INCORRECT_CONTROL_PARAMETERS;
        let status_text = StatusText::new(reason);
        let body = Vec::new();

        Self {
            status_code,
            status_text,
            body,
        }
    }

    pub fn socket_error(reason: impl ToString) -> Self {
        let status_code = StatusCode::SOCKET_ERROR;
        let status_text = StatusText::new(reason);
        let body = Vec::new();

        Self {
            status_code,
            status_text,
            body,
        }
    }

    pub fn face_destroyed(face_id: face::FaceId) -> Self {
        let face_id = tlv::Generic::from_tlv(face_id);
        let status_code = StatusCode::OK;
        let status_text = StatusText::from("DESTROYED");
        let body = vec![face_id];
        Self {
            status_code,
            status_text,
            body,
        }
    }

    pub fn into_result(self) -> io::Result<Self> {
        if self.status_code.is_ok() {
            Ok(self)
        } else {
            Err(io::Error::other(self.status_text.into_string()))
        }
    }
}
