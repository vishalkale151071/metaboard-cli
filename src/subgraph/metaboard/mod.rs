use std::fmt;

use clap::Parser;
use reqwest::Url;
use graphql_client::{GraphQLQuery, Response};
use rust_bigint::BigInt;
use serde_bytes::ByteBuf as Bytes;
use anyhow::anyhow;


use self::meta_board_query::MetaBoardQueryMetaBoard;


#[derive(GraphQLQuery, Debug)]
#[graphql(
    schema_path = "src/subgraph/metaboard/metaboard.schema.json",
    query_path = "src/subgraph/metaboard/metaboard.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize",
)]

pub struct MetaBoardQuery;

#[derive(Parser)]
pub struct Build {
    // subgraph api endpoint. if not given, local graph-node endpoint is used
    #[arg(short = 'e', long = "end_point", default_value= "http://localhost:8000/subgraphs/name/test/test")]
    end_point: Option<Url>,
    // metaboard contracts address
    #[arg(short, long = "meta_board")]
    meta_board_id: Option<String>
}

impl fmt::Debug for MetaBoardQueryMetaBoard {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut metas_id:String = String::new();
        if let Some(metas) = &self.metas {
            for meta in metas {
                    metas_id = meta.id.clone();
            }
        }
        f.debug_struct("MetaBoard")
            .field("id", &hex::encode(&self.id))
            .field("metaCount", &self.meta_count)
            .field("metas_id", &metas_id)
            .finish()
    }
}

pub async fn query(build: Build) -> anyhow::Result<()> {

    if build.meta_board_id.is_none() {
        return  Err(anyhow!(
            "metaboard contract address is not provided"
        ));
    }

    let url = Url::from(build.end_point.unwrap());

    let variables = meta_board_query::Variables{
        metaboard: build.meta_board_id
    };


    let request_body = MetaBoardQuery::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post((url).clone()).json(&request_body).send().await?;
    let response_body: Response<meta_board_query::ResponseData> = res.json().await?;
    match response_body {
        Response {
            data: Some(meta_board_query::ResponseData {meta_board}),
            ..
        } => {
            dbg!(&meta_board);
        },
        _ => { tracing::warn!("Failed to get metaboard"); }
    }

    Ok(())
}