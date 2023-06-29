use clap::Parser;
use reqwest::Url;
use graphql_client::{GraphQLQuery, Response};
use rust_bigint::BigInt;
use serde_bytes::ByteBuf as Bytes;
use anyhow::anyhow;
use serde::{Serialize, Deserialize};
use ethabi::{encode, Token};


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

#[derive(Serialize, Deserialize)]
struct MetaboardResponse {
    id: Bytes,
    address: Bytes,
    metaCount: BigInt,
    metas: Vec<String>
}

impl MetaBoardQueryMetaBoard {
    fn to_meta_board_response(&self) -> MetaboardResponse {
        let mut metas_id: Vec<String> = vec![];
        if let Some(metas) = &self.metas {
            for meta in metas {
                    metas_id.push(meta.id.clone());
            }
        }
        let response = MetaboardResponse {
            id: self.id.clone(),
            address: self.address.clone(),
            metaCount: self.meta_count.clone(),
            metas: metas_id
        };
        response
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
            let reponse = meta_board.unwrap().to_meta_board_response();
            let serialized = serde_json::to_string(&reponse).expect("Serialization failed");
            let encoded: Vec<u8> = encode(&[Token::String(serialized)]);
            print!("{:?}", encoded);
        },
        _ => { tracing::warn!("Failed to get metaboard"); }
    }

    Ok(())
}