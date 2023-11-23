use super::*;

#[derive(Debug)]
pub struct ControlResponse {
    status_code: StatusCode,
    status_text: StatusText,
    body: Vec<Box<dyn tlv::Tlv>>,
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
