use anyhow::anyhow;
use clap::Parser;
use std::{path::PathBuf, process::Command};

#[derive(Parser)]
pub struct Config {
    // name of subgraph user/subgraph-endpoint
    #[arg(short, long = "subgraph-name", default_value = "test/test")]
    subgraph_name: Option<String>,
    // subgraph version lable
    #[arg(short = 'v', long = "veriosn", default_value = "1")]
    version_lable: Option<String>,
    // blockchian network
    #[arg(short = 'n', long = "network", default_value = "localhost")]
    network: Option<String>,
    // subgraph template of subgraph.yaml
    #[arg(
        short = 't',
        long = "subgraph-template",
        default_value = "subgraph.template.yaml"
    )]
    subgraph_template: Option<PathBuf>,
    // contracts address
    #[arg(short = 'c', long = "contract-address")]
    contract_address: Option<String>,
    // block-number
    #[arg(short = 'b', long = "block-number", default_value = "0")]
    block_number: Option<String>,
    // graph access token
    #[arg(short = 'g', long = "graph-access-token")]
    graph_access_token: Option<String>,
    // endpoint
    #[arg(
        short = 'e',
        long = "end-point",
        default_value = "--node https://api.thegraph.com/deploy/"
    )]
    end_point: Option<String>,
}

pub async fn deploy(config: Config) -> anyhow::Result<()> {
    let graph_access_token = match config.graph_access_token {
        Some(token) => token,
        None => return Err(anyhow!("Graph Access Token is not proiveded.")),
    };

    if config.network.unwrap() != "localhost" {
        let output = Command::new(format!(
            "npx graph auth --product hosted-service {}",
            graph_access_token
        ))
        .output()?;
        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            println!("command error : {}", stderr)
        }
    }
    Ok(())
}
