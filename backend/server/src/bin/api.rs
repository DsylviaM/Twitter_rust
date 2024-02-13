use std::net::SocketAddr;
use color_eyre::{Result, Help, eyre::Context};

use clap::{command, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    // api -d <URL>
    // api --database-url
    #[clap(
        short,
        long,
        default_value = "postgres://test@localhost/test",
        env = "API_DATABASE_URL")]
    database_url: String,

    #[clap(short, long, default_value = "127.0.0.1:8070", env = "API_BIND")]
    bind: SocketAddr,

    #[clap(flatten)]
    verbosity: uchat_server::logging::Verbosity,
    
    #[command(subcommand)]
    command: Option<Command>
}

#[derive(Debug, Subcommand)]
enum Command {
    //api gen-key--help
    ///generate a session signing key
    GenKey,
}

async fn run() -> Result<()> {
    Ok(())
}

#[tokio::main]
async fn main() -> Result<()> {
    run().await
}