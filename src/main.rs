use std::net::IpAddr;

use clap::Parser;

use github_mock_api::MockServer;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: IpAddr,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().try_init()?;

    let args = Args::parse();

    let mut server = MockServer::start_on(args.host, args.port).await?;

    tracing::info!("Server running on {}", server.uri());

    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
