use super::*;

impl Type {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        VarNumber::from_u64(self.0).len()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }
}

impl From<u64> for Type {
    fn from(value: u64) -> Self {
        Self(value)
    }
}

impl From<Type> for u64 {
    fn from(value: Type) -> Self {
        value.0
    }
}
