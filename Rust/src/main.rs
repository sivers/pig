#[macro_use]
extern crate warp;
#[macro_use]
extern crate serde;

mod error;
mod pig;

use error::*;
use pig::Pig;

use warp::http::StatusCode;
use warp::Filter;
use warp::{Rejection, Reply};

fn validate_key(key: &str) -> bool {
    if key.len() != 4 {
        return false;
    }
    for c in key.chars() {
        if c < 'a' || c > 'z' {
            return false;
        }
    }
    true
}

async fn auth(key: String) -> Result<(Pig, i32), Rejection> {
    if !validate_key(&key) {
        return Err(Error("needs apikey header".into()).into());
    }
    let mut pig = Pig::new().await?;
    let person_id = pig.apikey_get(&key).await?.err_str("wrong apikey")?;
    Ok((pig, person_id))
}

async fn people_get((mut pig, person_id): (Pig, i32)) -> Result<impl Reply, Rejection> {
    pig.people_get().await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_header = warp::header::<String>("apikey")
        .or_else(|_| async move { Err(Error("needs apikey header".into()).into()) });

    // GET /
    let people_get = warp::get()
        .and(key_header.clone())
        .and_then(auth)
        .and_then(people_get);

    // GET /person/<id>
    // PATCH /person
    // GET /things
    // GET /thing/<id>
    // PATCH /thing/<id>
    // POST /things
    // DELETE /thing/<id>

    let routes = people_get;
    warp::serve(routes.recover(customize_error))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
