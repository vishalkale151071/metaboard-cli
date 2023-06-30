use clap::{Subcommand};

use crate::subgraph::metaboard::Build;


#[derive(Subcommand)]
#[command(about = "Interact with an order(s) onchain and offchain.")]
pub enum MetaBoard {
    #[command(about = "show metaboard entity")]
    Show(Build),
    #[command(about = "show metaboard entity")]
    Metas
}

pub async fn show(build: Build) -> anyhow::Result<()> {
    crate::subgraph::metaboard::query(build).await?;
    Ok(())
}