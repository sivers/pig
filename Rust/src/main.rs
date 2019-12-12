#[macro_use]
extern crate warp;
#[macro_use]
extern crate serde;

mod error;
mod pig;

use error::*;
use pig::Pig;

use std::collections::HashMap;
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

async fn people_get((mut pig, _person_id): (Pig, i32)) -> Result<impl Reply, Rejection> {
    pig.people_get().await
}

async fn things_get((mut pig, person_id): (Pig, i32)) -> Result<impl Reply, Rejection> {
    pig.things_get(person_id).await
}

async fn things_post(
    (mut pig, person_id): (Pig, i32),
    body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    if !body.contains_key("name") {
        return Err(Error("missing name".into()).into());
    }
    pig.thing_add(person_id, &body["name"]).await
}

async fn person_patch(
    (mut pig, person_id): (Pig, i32),
    body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    if !body.contains_key("name") {
        return Err(Error("missing name".into()).into());
    }
    pig.person_patch(person_id, &body["name"]).await
}

fn check_id(id: u32) -> Result<(), Error> {
    if id == 0 || id > 999999 {
        Err(Error("not found".into()))
    } else {
        Ok(())
    }
}

async fn person_get((mut pig, _person_id): (Pig, i32), id: u32) -> Result<impl Reply, Rejection> {
    check_id(id)?;
    pig.person_get(id as i32).await
}

async fn thing_get((mut pig, person_id): (Pig, i32), id: u32) -> Result<impl Reply, Rejection> {
    check_id(id)?;
    pig.thing_get(person_id, id as i32).await
}

async fn thing_patch(
    (mut pig, person_id): (Pig, i32),
    thing_id: u32,
    body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    check_id(thing_id)?;
    if !body.contains_key("name") {
        return Err(Error("missing name".into()).into());
    }
    pig.thing_patch(person_id, thing_id as i32, &body["name"]).await
}

async fn thing_delete(
    (mut pig, person_id): (Pig, i32),
    thing_id: u32,
) -> Result<impl Reply, Rejection> {
    check_id(thing_id)?;
    pig.thing_delete(person_id, thing_id as i32).await
}



#[tokio::main]
async fn main() -> Result<(), Error> {
    let key_header = warp::header::<String>("apikey")
        .or_else(|_| async move { Err(Error("needs apikey header".into()).into()) });
    let form_body = warp::body::form();

    // GET /
    let people_get = warp::get()
        .and(warp::path::end())
        .and(key_header.clone())
        .and_then(auth)
        .and_then(people_get);

    // GET /things
    let things_get = warp::get()
        .and(warp::path("things"))
        .and(key_header.clone())
        .and_then(auth)
        .and_then(things_get);

    // POST /things
    let things_post = warp::post()
        .and(warp::path("things"))
        .and(key_header.clone())
        .and_then(auth)
        .and(form_body)
        .and_then(things_post);

    // PATCH /person
    let person_patch = warp::patch()
        .and(warp::path("person"))
        .and(key_header.clone())
        .and_then(auth)
        .and(form_body)
        .and_then(person_patch);

    // GET /person/<id>
    let person_get = warp::get()
        .and(key_header.clone())
        .and_then(auth)
        .and(path!("person" / u32))
        .and_then(person_get);

    // GET /thing/<id>
    let thing_get = warp::get()
        .and(key_header.clone())
        .and_then(auth)
        .and(path!("thing" / u32))
        .and_then(thing_get);

    // PATCH /thing/<id>
    let thing_patch = warp::patch()
        .and(key_header.clone())
        .and_then(auth)
        .and(path!("thing" / u32))
        .and(form_body.clone())
        .and_then(thing_patch);

    // DELETE /thing/<id>
    let thing_delete = warp::delete()
        .and(key_header.clone())
        .and_then(auth)
        .and(path!("thing" / u32))
        .and_then(thing_delete);

    let routes = people_get
        .or(things_get)
        .or(things_post)
        .or(person_patch)
        .or(person_get)
        .or(thing_patch)
        .or(thing_delete)
        .or(thing_get);
    warp::serve(routes.recover(customize_error))
        .run(([127, 0, 0, 1], 3030))
        .await;

    Ok(())
}
