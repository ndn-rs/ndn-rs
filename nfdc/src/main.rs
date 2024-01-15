use std::io;

// use bytes::Bytes;
use clap::{Parser, Subcommand};
use tracing_subscriber::{fmt, EnvFilter};

use ndn::face;
use ndn::management as mgmt;
use ndn::router;
use ndn::tlv;

mod client;
mod mini;

#[derive(Debug, Parser)]
struct Cli {
    /// Use simple client by default
    #[arg(long, short)]
    simple: bool,
    /// RemoteUri
    #[arg(long, short, default_value = "tcp4://localhost:6363")]
    remote: face::Uri,
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    #[command(subcommand)]
    Channel(Channel),
    #[command(subcommand)]
    Face(Face),
    General,
    Router,
}

#[derive(Debug, Subcommand)]
enum Channel {
    List,
}

#[derive(Debug, Subcommand)]
enum Face {
    List,
}

impl Command {
    async fn execute(self, client: client::Client) -> anyhow::Result<()> {
        match self {
            Self::Channel(channel) => channel.execute(client).await,
            Self::Face(face) => face.execute(client).await,
            Self::General => self.general(client).await,
            Self::Router => self.router().await,
        }
    }

    async fn general(&self, mut client: client::Client) -> anyhow::Result<()> {
        let status = client
            .get::<mgmt::GeneralStatus>(mgmt::GeneralStatus::NAME)
            .await?;

        println!("STATUS\n{status:?}");
        println!("Start:   {}", status.start_timestamp.to_local_datetime());
        println!("Current: {}", status.current_timestamp.to_local_datetime());

        Ok(())
    }

    async fn router(&self) -> anyhow::Result<()> {
        let router = mini::Router::new().await?;
        router.info();
        let face = router.get_default_face().await;
        println!("{face:#}");

        let ping = tlv::Interest::new(mgmt::GeneralStatus::NAME)
            .must_be_fresh()
            .can_be_prefix();
        println!("{ping}");
        router.send_item(face, ping).await?;
        let generic = router.recv_item(face).await?;

        let data = tlv::Data::decode_from_generic(generic).expect("Should be valid data packet");
        println!("GOT PACKET\n{data:#}");
        let status = data
            .into_tlvcodec::<mgmt::GeneralStatus>()
            .expect("Should be valid General Status");
        println!("STATUS\n{status:?}");
        println!("Start:   {}", status.start_timestamp.to_local_datetime());
        println!("Current: {}", status.current_timestamp.to_local_datetime());

        Ok(())
    }
}

impl Channel {
    async fn execute(&self, mut client: client::Client) -> anyhow::Result<()> {
        match self {
            Self::List => {
                client
                    .get::<Vec<mgmt::ChannelStatus>>(mgmt::ChannelStatus::NAME)
                    .await?
                    .into_iter()
                    .for_each(|status| println!("{status:#}"));
            }
        }
        Ok(())
    }
}

impl Face {
    async fn execute(&self, mut client: client::Client) -> anyhow::Result<()> {
        match self {
            Self::List => {
                client
                    .get::<Vec<face::FaceStatus>>(face::FaceStatus::NAME)
                    .await?
                    .into_iter()
                    .for_each(|status| println!("{status}"));
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    fmt().with_env_filter(EnvFilter::from_default_env()).init();
    let cli = Cli::parse();
    let client = if cli.simple {
        client::Client::simple(cli.remote).await?
    } else {
        client::Client::multi(cli.remote).await?
    };
    cli.command.execute(client).await
}
