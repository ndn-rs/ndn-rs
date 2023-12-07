use super::*;

impl Type {
    #[allow(clippy::len_without_is_empty)]
    pub fn len(&self) -> usize {
        VarNumber::from_u64(self.0).len()
    }

    pub fn to_u64(&self) -> u64 {
        self.0
    }

    pub fn to_varnumber(&self) -> VarNumber {
        VarNumber::from_u64(self.0)
    }

    pub fn from_buf<B>(src: &mut B) -> Option<Self>
    where
        B: Buf,
    {
        VarNumber::from_buf(src).map(Self::from)
    }
}

impl From<VarNumber> for Type {
    fn from(n: VarNumber) -> Self {
        Self(n.to_u64())
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

impl str::FromStr for Type {
    type Err = NameError;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map(Self).map_err(|_| NameError::InvalidType)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
