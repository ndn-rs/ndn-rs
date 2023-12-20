use super::*;

mod impls;

pub trait TlvCodec: Sized {
    type Error: From<io::Error> + StdError + Send + Sync;

    fn total_size(&self) -> usize;

    fn encode(&self, dst: &mut BytesMut) -> Result<(), Self::Error>;

    fn decode(src: &mut BytesMut) -> Result<Self, Self::Error>;

    fn bytes(&self) -> Result<Bytes, Self::Error> {
        let mut dst = BytesMut::with_capacity(self.total_size());
        self.encode(&mut dst)?;
        Ok(dst.freeze())
    }
}
