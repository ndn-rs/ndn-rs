use super::*;

#[derive(Debug)]
pub struct ControlResponse {
    pub status_code: StatusCode,
    pub status_text: StatusText,
    pub body: Vec<Box<dyn tlv::Tlv>>,
}

impl ControlResponse {
    pub fn incorrect_control_parameters(reason: impl Into<String>) -> Self {
        let status_code = StatusCode::incorrect_control_parameters();
        let status_text = StatusText::from(reason);
        let body = Vec::new();

        Self {
            status_code,
            status_text,
            body,
        }
    }

    pub fn socket_error(reason: impl ToString) -> Self {
        let status_code = StatusCode::socket_error();
        let status_text = StatusText::from(reason.to_string());
        let body = Vec::new();

        Self {
            status_code,
            status_text,
            body,
        }
    }
}

impl tlv::Tlv for ControlResponse {
    fn r#type(&self) -> tlv::Type {
        tlv::Type::ControlResponse
    }

    fn value(&self) -> Option<Bytes> {
        let items = [self.status_code.value(), self.status_text.value()]
            .into_iter()
            .chain(self.body.iter().map(|item| item.value()));
        tlv::collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        todo!()
    }
}
