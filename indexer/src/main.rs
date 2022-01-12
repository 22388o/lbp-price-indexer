use crate::models::{Response, ReverseSimulationResponse};
use actix_web::rt::{spawn, time};
use actix_web::{App, HttpServer};
use anyhow::Result;
use mysql::prelude::Queryable;
use mysql::PooledConn;
use mysql::{params, OptsBuilder, Pool};
use serde_json::{json, to_string};
use std::time::Duration;
use terra_rust_api::Terra;
use std::env::{var};

pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pass = match var("DBPASS") {
        Ok(pass) => {
            Some(pass)
        }
        Err(_) => {
            None
        }
    };
    let mysql_pool = Pool::new(
        OptsBuilder::new()
            .user(Some(var("DBUSER").unwrap_or("root".to_string())))
            .pass(pass)
            .db_name(Some(var("DBNAME").unwrap_or("indexer".to_string()).to_string())),
    )
    .unwrap();

    let lcd_url = var("LCDURL").unwrap();
    let chain_id = var("CHAINID").unwrap();

    let terra = Terra::lcd_client_no_tx(lcd_url.as_str(), chain_id.as_str());
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        let mut last_height = 0;
        loop {
            let last_block = &terra.tendermint().blocks().await;
            match last_block {
                Ok(block_result) => {
                    let block_height = block_result.block.header.height;
                    if block_height > last_height {
                        last_height = block_height.clone();
                        let block_time = block_result.block.header.time.timestamp() as u64;
                        let conn = mysql_pool.get_conn().unwrap();
                        query_reverse_simulation(&terra, block_time, conn).await;
                    }
                }
                Err(_) => {
                    println!("Couldn't fetch latest block")
                }
            }

            interval.tick().await;
            println!("tick");
        }
    });
    HttpServer::new(move || App::new())
        .bind("0.0.0.0:1337")?
        .run()
        .await
}

async fn query_reverse_simulation(terra: &Terra, block_time: u64, mut mysql_conn: PooledConn) {
    let pool_addr = var("POOL").unwrap();
    let token_addr = var("TOKEN").unwrap();
    let query_msg = json!({
        "reverse_simulation" : {
            "ask_asset": {
                "info": {
                    "token": {
                        "contract_addr": token_addr.clone()
                    }
                },
                "amount": "1000000"
            },
            "block_time": block_time.clone()
        }
    });

    let json_query = to_string(&query_msg).unwrap();
    let result: Result<Response<ReverseSimulationResponse>> =
        terra.wasm().query(pool_addr.as_str(), json_query.as_str()).await;

    match result {
        Ok(response) => {
            println!("Query Success.");
            let result = &response.result;
            let mysql_result = mysql_conn.exec_drop(
                r"INSERT INTO reverse_simulation 
                (height, offer_amount, spread_amount, commission_amount, ask_weight, offer_weight, block_time) 
                VALUES (:height, :offer_amount, :spread_amount, :commission_amount, :ask_weight, :offer_weight, :block_time)",
                params! {
                    "height" => &response.height,
                    "offer_amount" => &result.offer_amount.to_string(),
                    "spread_amount" => &result.spread_amount.to_string(),
                    "commission_amount" => &result.commission_amount.to_string(),
                    "ask_weight" => &result.ask_weight,
                    "offer_weight" => &result.offer_weight,
                    "block_time" => block_time.clone()
                }
            );
            println!("Insert result: {:?}", mysql_result);
        }
        Err(e) => {
            println!("Query Error: {:?}", e);
        }
    }
}

//Query for the API:
//SELECT height, offer_amount, (`block_time` - `block_time`%60) * 1000 time FROM `reverse_simulation` GROUP BY time ORDER BY time DESC;
