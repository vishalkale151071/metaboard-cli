use std::str::FromStr;

use clap::Parser;
use reqwest::Url;
use graphql_client::{GraphQLQuery, Response};
use rust_bigint::BigInt;
use serde_bytes::ByteBuf as Bytes;
use ethabi::{encode, Token, ethereum_types::H160, Uint};

#[derive(GraphQLQuery)]
#[derive(Debug)]
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
    #[arg(short = 'm', long = "meta_board")]
    meta_board_id: Option<String>
}


pub async fn query(build: Build) -> anyhow::Result<()> {
   
    let url = Url::from(build.end_point.unwrap());

    let variables = meta_board_query::Variables {
        metaboard: build.meta_board_id,
    };

    let request_body = MetaBoardQuery::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_board_query::ResponseData> = res.json().await?;

    if let Some(meta_board) = response_body.data.and_then(|data| data.meta_board) {
        let id = String::from_utf8(meta_board.id.to_vec()).unwrap();
        let address = String::from_utf8(meta_board.address.to_vec()).unwrap();

        let mut meta_list: Vec<Token> = vec![];
        if let Some(metas) = meta_board.metas {
            meta_list = metas.into_iter().map(|meta| Token::String(meta.id)).collect();
        }

        let encoded = encode(&[
            Token::Address(H160::from_str(&id)?),
            Token::Address(H160::from_str(&address)?),
            Token::Uint(Uint::from_str(&meta_board.meta_count.to_string())?),
            Token::Array(meta_list),
        ]);

        println!("{}", hex::encode(encoded));
    } else {
        tracing::warn!("Failed to get metaboard");
    }

    Ok(())
}
