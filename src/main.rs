use crate::models::{Response, ReverseSimulationResponse};
use actix_web::rt::{spawn, time};
use actix_web::{App, HttpServer};
use anyhow::Result;
use cosmwasm_std::ContractResult;
use serde_json::{json, to_string, Map};
use std::any::Any;
use std::time::Duration;
use terra_rust_api::Terra;

pub mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let terra = Terra::lcd_client_no_tx("http://143.244.190.1:3060", "localterra");
    spawn(async move {
        let mut interval = time::interval(Duration::from_secs(3));
        // let mut last_height = 0;
        loop {
            let last_block = &terra.tendermint().blocks().await;
            match last_block {
                Ok(block) => {
                    //Get Height, if diff than last height, query ReverseSimulation and store it. :)
                    println!("Last Block {:?}", block);
                    let block_time = block.block.header.time.timestamp().unsigned_abs();
                    query_reverse_simulation(&terra, block_time).await;
                    println!("Block fetched.")
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

async fn query_reverse_simulation(terra: &Terra, block_time: u64) {
    let pool_addr = "terra106h80nqa9k7xclnzxssqjuhjk9lh683p8dmvxw";
    let token_addr = "terra1ftscx7hy4qeqrrc6wx7myk2ftwf6z3n766v9fv";
    let query_msg = json!({
        "reverse_simulation" : {
            "ask_asset": {
                "info": {
                    "token": {
                        "contract_addr": token_addr
                    }
                },
                "amount": "1"
            },
            "block_time": block_time
        }
    });
    let json_query = to_string(&query_msg).unwrap();
    println!("Execute Query: {:?}", json_query.as_str());
    let result: Result<Response<ReverseSimulationResponse>> =
        terra.wasm().query(pool_addr, json_query.as_str()).await;
    match result {
        Ok(response) => {
            println!("Query Success: {:?}", response);
        }
        Err(e) => {
            println!("Query Error: {:?}", e);
        }
    }
}
