use super::*;

#[derive(Debug)]
pub struct NameComponentIterator {
    pub(super) value: Bytes,
}

impl Iterator for NameComponentIterator {
    type Item = Result<NameComponent, DecodeError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.value.is_empty() {
            None
        } else {
            todo!()
        }
    }
}
