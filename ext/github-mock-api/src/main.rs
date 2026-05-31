mod seed;

use std::net::IpAddr;
use std::path::PathBuf;

use clap::Parser;

use github_mock_api::MockServer;

use crate::seed::load_repositories;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "127.0.0.1")]
    host: IpAddr,
    #[arg(short, long, default_value_t = 3000)]
    port: u16,
    #[arg(long, value_name = "DIR", default_value = "seed")]
    data_dir: PathBuf,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    tracing_subscriber::fmt().try_init()?;

    let args = Args::parse();

    let mut server = MockServer::start_on(args.host, args.port).await?;

    if let Some(repositories) = load_repositories(&args.data_dir)? {
        for repo in repositories {
            let full_name = repo.full_name.clone();
            server.add_repository(repo).await;
            tracing::info!("Registered repository {}", full_name);
        }
    }

    tracing::info!("Server running on {}", server.uri());

    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
