use crate::{mobc_pool::MobcPool, r2d2_pool::R2D2Pool};
use std::convert::Infallible;
use warp::Filter;

pub fn with_redis_client(
    client: redis::Client,
) -> impl Filter<Extract = (redis::Client,), Error = Infallible> + Clone {
    warp::any().map(move || client.clone())
}

pub fn with_mobc_pool(
    pool: MobcPool,
) -> impl Filter<Extract = (MobcPool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}

pub fn with_r2d2_pool(
    pool: R2D2Pool,
) -> impl Filter<Extract = (R2D2Pool,), Error = Infallible> + Clone {
    warp::any().map(move || pool.clone())
}
