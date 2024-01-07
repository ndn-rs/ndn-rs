use super::*;

impl Type {
    pub const fn new(n: u64) -> Self {
        Self(n)
    }

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

    pub fn peek(buf: &[u8]) -> Option<Self> {
        VarNumber::peek(buf).map(Self::from)
    }

    pub fn decode(src: &mut BytesMut) -> Option<Self> {
        Self::from_buf(src)
    }
}

impl From<VarNumber> for Type {
    fn from(n: VarNumber) -> Self {
        Self(n.to_u64())
    }
}

impl From<Type> for VarNumber {
    fn from(value: Type) -> Self {
        value.to_varnumber()
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
    type Err = <u64 as str::FromStr>::Err;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        text.parse().map(Self)
    }
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
