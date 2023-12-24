use super::*;

#[derive(Debug)]
pub struct Tcp {
    framed: Framed<net::TcpStream, TlvCodec>,
}

impl Tcp {
    pub(super) async fn new(
        _local: impl net::ToSocketAddrs,
        remote: impl net::ToSocketAddrs,
    ) -> io::Result<Self> {
        tracing::info!("Ignoring local for now");
        let socket = net::TcpStream::connect(remote).await?;
        let codec = TlvCodec::new();
        let framed = Framed::new(socket, codec);
        Ok(Self { framed })
    }

    fn local_addr(&self) -> io::Result<SocketAddr> {
        self.framed.get_ref().local_addr()
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Tcp::PREFIX,
            face::URI_DELIMITER,
            self.local_addr()?,
        );
        Ok(uri)
    }

    pub(super) async fn send_item(&mut self, item: impl tlv::Tlv) -> io::Result<()> {
        self.framed.send(item).await
    }

    pub(super) async fn recv_item(&mut self) -> io::Result<Option<tlv::Generic>> {
        self.framed.try_next().await
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        let count = loop {
            self.framed.get_ref().writable().await?;

            // Try to write data, this may still fail with `WouldBlock`
            // if the readiness event is a false positive.
            match self.framed.get_ref().try_write(&bytes) {
                Ok(count) => break count,
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => {
                    return Err(e);
                }
            }
        };

        if count == bytes.len() {
            Ok(())
        } else {
            Err(io::Error::other("Failed to send TCP data"))
        }
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn recv(&self, mut bytes: BytesMut) -> io::Result<Bytes> {
        loop {
            self.framed.get_ref().readable().await?;

            let mut buf = [0; 8800];
            tracing::trace!(buffer = buf.len(), "Got buffer");

            match self.framed.get_ref().try_read(&mut buf) {
                Ok(0) => break,
                Ok(count) => {
                    tracing::trace!(count, "Got bytes");
                    bytes.extend(&buf[..count]);
                    if tlv::Generic::from_buf(&mut buf.as_ref()).is_some() {
                        break;
                    }
                    // let generic = tlv::Generic::from_buf(&mut buf.as_ref()).unwrap();
                    // println!("{generic:?}");
                    // generic
                    //     .items()
                    //     .unwrap_or_default()
                    //     .into_iter()
                    //     .for_each(|item| println!("{item:?}"));
                    // println!("{}", String::from_utf8_lossy(&buf[..count]));
                }
                Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => continue,
                Err(e) => return Err(e),
            }
        }

        Ok(bytes.freeze())
    }
}
