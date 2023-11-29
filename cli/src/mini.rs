use super::*;

#[derive(Debug)]
pub(crate) struct Router {
    router: router::Router,
}

impl Router {
    pub(crate) async fn new() -> anyhow::Result<Self> {
        let router = router::Router::new();
        let params = Self::tcp_face();
        let response = router.handle_create_face(params).await;
        anyhow::ensure!(response.status_code.is_ok(), response.status_text);
        Ok(Self { router })
    }

    fn tcp_face() -> mgmt::ControlParameters {
        mgmt::ControlParameters::create_face("tcp://127.0.0.1:6363")
    }

    pub(crate) fn info(&self) {
        println!("{:#?}", self.router);
    }
}
