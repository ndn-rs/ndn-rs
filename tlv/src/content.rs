use super::*;

octets!(Content => Type::Content);

#[derive(Debug)]
pub struct Iter {
    bytes: Bytes,
}

impl Iterator for Iter {
    type Item = Generic;

    fn next(&mut self) -> Option<Self::Item> {
        let item = Generic::from_bytes(&mut self.bytes)?;
        Some(item)
    }
}

impl IntoIterator for Content {
    type Item = Generic;

    type IntoIter = Iter;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter { bytes: self.0 }
    }
}
