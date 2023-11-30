use super::*;

pub use block::FinalBlockId;

mod block;

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Name {
    components: Vec<NameComponent>,
}

impl Name {
    pub fn generic(name: impl Into<String>) -> Self {
        let components = name
            .into()
            .split('/')
            .filter(|item| !item.is_empty())
            .map(GenericNameComponent::from)
            .map(NameComponent::from)
            .collect();
        // let name = GenericNameComponent::from(name);
        // let components = vec![name.into()];
        Self { components }
    }

    pub fn digest(digest: [u8; 32]) -> Self {
        let digest = ImplicitSha256DigestComponent::new(digest);
        let components = vec![digest.into()];
        Self { components }
    }
}

impl Tlv for Name {
    fn r#type(&self) -> Type {
        Type::Name
    }

    fn value(&self) -> Option<Bytes> {
        let items = self.components.iter().map(|component| component.bytes());
        collect_to_bytes(items)
    }

    fn payload_size(&self) -> usize {
        self.components
            .iter()
            .map(|component| component.size())
            .sum()
    }
}

impl fmt::Display for Name {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let components = self
            .components
            .iter()
            .map(|component| component.to_string())
            .collect::<Vec<_>>()
            .join(",");
        format_args!("<Name>[{}]", components).fmt(f)
    }
}
