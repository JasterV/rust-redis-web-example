use warp::Reply;

use crate::{
    direct,
    mobc_pool::{self, MobcPool},
    models::WebResult,
    r2d2_pool::{self, R2D2Pool},
};

pub async fn direct_handler(client: redis::Client) -> WebResult<impl Reply> {
    let mut con = direct::get_con(client)
        .await
        .map_err(warp::reject::custom)?;
    direct::set_str(&mut con, "hello", "direct_world", 60)
        .await
        .map_err(warp::reject::custom)?;
    let value = direct::get_str(&mut con, "hello")
        .await
        .map_err(warp::reject::custom)?;
    Ok(value)
}

pub async fn r2d2_handler(pool: R2D2Pool) -> WebResult<impl Reply> {
    r2d2_pool::set_str(&pool, "r2d2_hello", "r2d2_world", 60)
        .map_err(|e| warp::reject::custom(e))?;
    let value = r2d2_pool::get_str(&pool, "r2d2_hello").map_err(|e| warp::reject::custom(e))?;
    Ok(value)
}

pub async fn mobc_handler(pool: MobcPool) -> WebResult<impl Reply> {
    mobc_pool::set_str(&pool, "mobc_hello", "mobc_world", 60)
        .await
        .map_err(|e| warp::reject::custom(e))?;
    let value = mobc_pool::get_str(&pool, "mobc_hello")
        .await
        .map_err(|e| warp::reject::custom(e))?;
    Ok(value)
}
