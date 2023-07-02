use anyhow::anyhow;
use clap::Parser;
use mustache::MapBuilder;
use std::{fs, path::PathBuf, process::Command, str::FromStr};

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
        default_value = "http://localhost:8020/"
    )]
    end_point: Option<String>,
    // output path
    #[arg(short = 'o', long = "output-path", default_value = "subgraph.yaml")]
    output_path: Option<PathBuf>,
    // Root dir
    #[arg(short = 'r', long = "root", default_value = "/")]
    root_dir: Option<PathBuf>,
}

pub async fn deploy(config: Config) -> anyhow::Result<()> {
    let network = match config.network {
        Some(network) => network,
        None => return Err(anyhow!("No network provided")),
    };

    let contract = match config.contract_address {
        Some(address) => address,
        None => return Err(anyhow!("No contract address provided")),
    };

    let block_number = match config.block_number {
        Some(block) => block,
        None => return Err(anyhow!("No block-number provided")),
    };

    let output_path = match config.output_path {
        Some(path) => path,
        None => return Err(anyhow!("No output path provided")),
    };

    let subgraph_template = match config.subgraph_template {
        Some(path) => path,
        None => return Err(anyhow!("No subgraph-template path provided")),
    };

    let root_dir = match config.root_dir {
        Some(path) => path,
        None => return Err(anyhow!("No root path provided")),
    };

    let end_point = match config.end_point {
        Some(val) => val,
        None => return Err(anyhow!("No end-point provided")),
    };

    let subgraph_name = match config.subgraph_name {
        Some(name) => name,
        None => return Err(anyhow!("No subgraph-name provided provided")),
    };

    let version_lable = match config.version_lable {
        Some(label) => label,
        None => return Err(anyhow!("No version-lable provided provided")),
    };

    if network.clone().ne(&String::from_str("localhost").unwrap()) {
        let graph_access_token = match config.graph_access_token {
            Some(token) => token,
            None => return Err(anyhow!("Graph Access Token is not proiveded.")),
        };

        let output = Command::new("bash")
            .args(&[
                "-c",
                &format!(
                    "npx graph auth --product hosted-service {}",
                    graph_access_token
                ),
            ])
            .output()
            .expect("Failed graph auth command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    }

    let data = MapBuilder::new()
        .insert_str("network", &network)
        .insert_str("MetaBoard", contract)
        .insert_str("MetaBoardBlock", block_number)
        .build();

    let template = fs::read_to_string(subgraph_template.clone()).expect(&format!(
        "Fail to read {}",
        subgraph_template.to_str().unwrap()
    ));

    let renderd = mustache::compile_str(&template)
        .expect("Failed to compile template")
        .render_data_to_string(&data)
        .expect("Failed to render template");

    let _write = fs::write(output_path, renderd)?;

    let output = Command::new("bash")
        .current_dir(&root_dir)
        .args(&["-c", "npx graph codegen && npx graph build"])
        .output()?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        println!("{}", stdout);
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(anyhow!("{}", stderr));
    }

    if network.ne(&String::from_str("localhost").unwrap()) {
        let output = Command::new("bash")
            .current_dir(&root_dir)
            .args(&[
                "-c",
                &format!("npx graph deploy {} {}", end_point, subgraph_name),
            ])
            .output()
            .expect("Failed graph deploy command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    } else {
        let _output = Command::new("bash")
            .args(&[
                "-c",
                &format!("graph create --node {} {}", end_point, subgraph_name),
            ])
            .output()
            .expect("Failed local deploy command");

        let output = Command::new("bash")
            .current_dir(&root_dir)
            .args(&[
                "-c",
                &format!(
                    "graph deploy --node {} --ipfs http://localhost:5001 {}  --version-label {}",
                    end_point, subgraph_name, version_lable
                ),
            ])
            .output()
            .expect("Failed local deploy command");

        if output.status.success() {
            let stdout = String::from_utf8_lossy(&output.stdout);
            println!("{}", stdout);
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(anyhow!("{}", stderr));
        }
    }

    Ok(())
}
