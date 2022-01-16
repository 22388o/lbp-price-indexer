use actix_cors::Cors;
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer, Responder};
use mysql::prelude::Queryable;
use mysql::{OptsBuilder, Pool};
use serde::{Deserialize, Serialize};
use std::env::var;
use std::time::Duration;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pass = match var("DBPASS") {
        Ok(pass) => Some(pass),
        Err(_) => None,
    };
    let mysql_pool = Pool::new(
        OptsBuilder::new()
            .tcp_connect_timeout(Some(Duration::from_secs(10)))
            .user(Some(var("DBUSER").unwrap_or("root".to_string())))
            .ip_or_hostname(Some(var("DBHOST").unwrap_or("127.0.0.1".to_string())))
            .pass(pass)
            .db_name(Some(
                var("DBNAME").unwrap_or("indexer".to_string()).to_string(),
            )),
    )
    .unwrap();

    HttpServer::new(move || {
        let cors = Cors::permissive().allowed_methods(vec!["GET"]);
        App::new()
            .wrap(cors)
            .app_data(web::Data::new(mysql_pool.clone()))
            .route("/", web::get().to(query))
    })
    .bind(("0.0.0.0", 80))?
    .run()
    .await
}

async fn query(_req: HttpRequest, data: web::Data<Pool>) -> impl Responder {
    let conn = &mut data.get_conn().unwrap();
    let result: mysql::Result<Vec<QueryResult>> = conn.query_map(
        "(SELECT CAST(AVG(offer_amount) AS INT) as offer_amount, height, (`block_time` - `block_time`%60) * 1000 time FROM `reverse_simulation` GROUP BY time)
                UNION
                (SELECT CAST(offer_amount AS INT), height,  `block_time` * 1000 time FROM `reverse_simulation` GROUP BY time ORDER BY block_time DESC LIMIT 1)
                ORDER BY time ASC;",
        |(offer_amount, height, time)| QueryResult {
            offer_amount,
            height,
            time,
        },
    );
    HttpResponse::Ok()
        .content_type("application/json")
        .json(result.unwrap())
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct QueryResult {
    height: u64,
    offer_amount: u64,
    time: u64,
}
