use anyhow::Result;
use clap::command;
use clap::{Parser, Subcommand};
use crate::subgraph::metaboard::Build;

mod metaboard;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    metaboard: MetaBoardOption,
}

#[derive(Subcommand)]
pub enum MetaBoardOption {
    MetaBoard(Build)
}

pub async fn dispatch(metaboard: MetaBoardOption) -> Result<()> {
    match metaboard {
        MetaBoardOption::MetaBoard(build) => metaboard::show(build).await
    }
}

pub async fn main() -> Result<()> {
    tracing::subscriber::set_global_default(tracing_subscriber::fmt::Subscriber::new())?;

    let cli = Cli::parse();
    dispatch(cli.metaboard).await
}
