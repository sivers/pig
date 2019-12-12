#[macro_use]
extern crate warp;
#[macro_use]
extern crate serde;

use warp::Filter;

#[tokio::main]
async fn main() {
    let routes = warp::get().map(|| "Hello, World!");
    warp::serve(routes.recover(customize_error))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

use warp::http::StatusCode;
use warp::{reject, Rejection, Reply};

async fn customize_error(err: Rejection) -> Result<impl Reply, Rejection> {
    if err.is_not_found() {
        #[derive(Serialize)]
        struct Empty {}
        let json = warp::reply::json(&Empty {});
        Ok(warp::reply::with_status(json, StatusCode::NOT_FOUND))
    } else {
        Err(err)
    }
}
