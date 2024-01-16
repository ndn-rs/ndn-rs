use super::*;

mod impls;

pub trait TlvCodec: Sized {
    type Error: From<io::Error> + Into<io::Error> + StdError + Send + Sync;
    const TYPE: Type;

    fn total_size(&self) -> usize;

    fn encode(&self, dst: &mut BytesMut);

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error>;

    fn bytes(&self) -> Bytes {
        let mut dst = BytesMut::with_capacity(self.total_size());
        self.encode(&mut dst);
        dst.freeze()
    }
}
