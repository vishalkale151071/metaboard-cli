use clap::Subcommand;

#[derive(Subcommand)]
#[command(about = "wait for subgraph to sync")]
pub enum Wait {
    #[command(about = "wait for subgraph to sync")]
    Wait
}

pub async fn wait() -> anyhow::Result<()> {
    crate::subgraph::wait::wait().await?;
    Ok(())
}