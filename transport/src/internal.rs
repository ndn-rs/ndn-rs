use tokio::sync::mpsc;
use tokio::sync::Mutex;

use super::*;

#[derive(Debug)]
pub struct Internal {
    tx: mpsc::Sender<Bytes>,
    rx: Mutex<mpsc::Receiver<Bytes>>,
}

impl Internal {
    pub(super) async fn new() -> io::Result<Self> {
        let (tx, rx) = mpsc::channel(16);
        let rx = Mutex::new(rx);
        Ok(Self { tx, rx })
    }

    fn local_addr(&self) -> &'static str {
        ""
    }

    pub(super) fn face_uri(&self) -> io::Result<String> {
        let uri = format!(
            "{}{}{}",
            face::Internal::PREFIX,
            face::URI_DELIMITER,
            self.local_addr(),
        );
        Ok(uri)
    }

    pub(super) async fn send_item(&mut self, item: impl tlv::Tlv) -> io::Result<()> {
        let _ = item;
        todo!()
    }

    pub(super) async fn recv_item(&mut self) -> io::Result<Option<tlv::Generic>> {
        Ok(None)
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn send(&self, bytes: Bytes) -> io::Result<()> {
        self.tx.send(bytes).await.map_err(io::Error::other)
    }

    #[tracing::instrument(level = "trace", skip_all, err(level = "error"))]
    pub(super) async fn recv(&self, _bytes: BytesMut) -> io::Result<Bytes> {
        self.rx
            .lock()
            .await
            .recv()
            .await
            .ok_or_else(|| io::Error::other("channel has been closed"))
    }
}
