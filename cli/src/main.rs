use bytes::Bytes;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

use ndn::face;
use ndn::management as mgmt;
use ndn::router;
use ndn::tlv;

mod mini;

#[derive(Debug, Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    Ping,
}

impl Command {
    async fn execute(self) -> anyhow::Result<()> {
        let router = mini::Router::new().await?;
        router.info();
        let face = router.get_default_face().await;
        println!("{face:#}");

        let ping = tlv::Interest::new("/localhost/nfd/status/general")
            .must_be_fresh()
            .can_be_prefix();
        println!("{ping}");
        router.send(&face, ping).await?;
        let data = router.recv(&face).await?;

        println!("{data:#?}");

        Ok(())
    }
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let cli = Cli::parse();
    cli.command.execute().await
}
