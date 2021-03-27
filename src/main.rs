mod constants;
mod handlers;
mod lib;
mod middleware;
mod models;

use constants::REDIS_CON_STRING;
use handlers::{direct_handler, mobc_handler, r2d2_handler};
use lib::{direct, mobc_pool, r2d2_pool};
use middleware::{with_mobc_pool, with_r2d2_pool, with_redis_client};
use warp::Filter;

#[tokio::main]
async fn main() {
    let redis_client = redis::Client::open(REDIS_CON_STRING).expect("can create redis client");
    let mobc_pool = mobc_pool::connect(REDIS_CON_STRING)
        .await
        .expect("can create mobc pool");
    let r2d2_pool = r2d2_pool::connect(REDIS_CON_STRING).expect("can create r2d2 pool");

    let direct_route = warp::path!("direct")
        .and(with_redis_client(redis_client.clone()))
        .and_then(direct_handler);

    let r2d2_route = warp::path!("r2d2")
        .and(with_r2d2_pool(r2d2_pool.clone()))
        .and_then(r2d2_handler);

    let mobc_route = warp::path!("mobc")
        .and(with_mobc_pool(mobc_pool.clone()))
        .and_then(mobc_handler);

    let routes = mobc_route.or(direct_route).or(r2d2_route);
    warp::serve(routes).run(([0, 0, 0, 0], 8080)).await;
}
