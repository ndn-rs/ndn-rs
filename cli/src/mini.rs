use super::*;

#[derive(Debug)]
pub(crate) struct Router {
    router: router::Router,
}

impl Router {
    pub(crate) async fn new() -> anyhow::Result<Self> {
        let router = router::Router::new();
        for uri in [
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
            "tcp4://anchor.local:6363",
        ] {
            let params = Self::tcp_face(uri);
            let response = router.handle_create_face(params).await;
            anyhow::ensure!(response.status_code.is_ok(), response.status_text);
        }
        Ok(Self { router })
    }

    fn tcp_face(uri: &str) -> mgmt::ControlParameters {
        // mgmt::ControlParameters::create_face("tcp://127.0.0.1:6363")
        mgmt::ControlParameters::create_face(uri)
    }

    pub(crate) fn info(&self) {
        println!("{:#?}", self.router);
    }
}
