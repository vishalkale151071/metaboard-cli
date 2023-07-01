use std::str::FromStr;

use anyhow::anyhow;
use clap::Parser;
use reqwest::Url;
use graphql_client::{GraphQLQuery, Response};
use rust_bigint::BigInt;
use serde_bytes::ByteBuf as Bytes;
use ethabi::{encode, Token, ethereum_types::H160, Uint};

#[derive(GraphQLQuery)]
#[derive(Debug)]
#[graphql(
    schema_path = "src/subgraph/query/schema.json",
    query_path = "src/subgraph/query/metav1/query.graphql",
    reseponse_derives = "Debug, Serialize, Deserialize",
)]

pub struct MetaV1;

#[derive(Parser)]
pub struct MetaV1Struct {
    // subgraph api endpoint. if not given, local graph-node endpoint is used
    #[arg(short = 'e', long = "end_point", default_value= "http://localhost:8000/subgraphs/name/test/test")]
    end_point: Option<Url>,
    // metaboard contracts address
    #[arg(short = 't', long = "transaction_hash")]
    transaction_hash: Option<String>
}


pub async fn query(build: MetaV1Struct) -> anyhow::Result<()> {
   
    let url = Url::from(build.end_point.unwrap());
    let transaction_hash = match build.transaction_hash {
        Some(hash) => hash,
        None => return Err(anyhow!("No transaction-hash provided"))
    };

    let variables = meta_v1::Variables {
        trx_hash: transaction_hash.into(),
    };

    let request_body = MetaV1::build_query(variables);
    let client = reqwest::Client::new();
    let res = client.post(url.clone()).json(&request_body).send().await?;
    let response_body: Response<meta_v1::ResponseData> = res.json().await?;

    if let Some(meta_v1) = response_body.data.and_then(|data| data.meta_v1) {
        let id = meta_v1.id;
        let sender= String::from_utf8(meta_v1.sender.to_vec()).unwrap();
        let meta = meta_v1.meta.to_vec();
        let subject= meta_v1.subject.to_string();
        let magic_number= meta_v1.magic_number.to_string();
        let payload= meta_v1.payload;
        let content_type= meta_v1.content_type;
        let meta_board = String::from_utf8(meta_v1.meta_board.id.to_vec()).unwrap();

        let encoded = encode(&[
            Token::String(id),
            Token::Address(H160::from_str(&sender)?),
            Token::Bytes(meta),
            Token::Uint(Uint::from_str(&subject)?),
            Token::Uint(Uint::from_str(&magic_number)?),
            Token::String(payload),
            Token::String(content_type),
            Token::Address(H160::from_str(&meta_board)?)
        ]);

        println!("{}", hex::encode(encoded));
    } else {
        tracing::warn!("Failed to get metaboard");
    }

    Ok(())
}
