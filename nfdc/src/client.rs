use ndn::client::concurrent;
use ndn::client::simple;

use super::*;

#[derive(Debug)]
pub(super) struct Client {
    inner: Inner,
}

impl Client {
    pub(super) async fn simple(remote: impl Into<face::Uri>) -> io::Result<Self> {
        Inner::simple(remote).await.map(|inner| Self { inner })
    }

    pub(super) async fn multi(remote: impl Into<face::Uri>) -> io::Result<Self> {
        Inner::multi(remote).await.map(|inner| Self { inner })
    }

    pub(super) async fn get<T>(&mut self, name: impl AsRef<str>) -> Result<T, T::Error>
    where
        T: tlv::TlvCodec,
    {
        match self.inner {
            Inner::Simple(ref mut client) => client.get::<T>(name).await,
            Inner::Multi(ref client) => client.express_interest::<T>(name).await?.data().await,
        }
    }
}

#[derive(Debug)]
enum Inner {
    Simple(client::simple::Client),
    Multi(client::concurrent::Client),
}

impl Inner {
    async fn simple(remote: impl Into<face::Uri>) -> io::Result<Self> {
        simple::Client::new(remote).await.map(Self::Simple)
    }

    async fn multi(remote: impl Into<face::Uri>) -> io::Result<Self> {
        concurrent::Client::new(remote).await.map(Self::Multi)
    }
}
