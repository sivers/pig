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
        Err(custom_err(401, "needs apikey header"))?;
    }
    let mut pig = Pig::new().await?;
    let person_id = match pig.apikey_get(&key).await? {
        Some(person_id) => person_id,
        None => return Err(custom_err(401, "wrong apikey").into()),
    };
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
        Err(custom_err(412, "missing name"))?;
    }
    pig.thing_add(person_id, &body["name"]).await
}

async fn person_patch(
    (mut pig, person_id): (Pig, i32),
    body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    if !body.contains_key("name") {
        Err(custom_err(412, "missing name"))?;
    }
    pig.person_patch(person_id, &body["name"]).await
}

fn check_id(id: u32) -> Result<(), Error> {
    if id == 0 || id > 999999 {
        Err(custom_err(404, ""))
    } else {
        Ok(())
    }
}

async fn person_get(id: u32, (mut pig, _person_id): (Pig, i32)) -> Result<impl Reply, Rejection> {
    check_id(id)?;
    pig.person_get(id as i32).await
}

async fn thing_get(id: u32, (mut pig, person_id): (Pig, i32)) -> Result<impl Reply, Rejection> {
    check_id(id)?;
    pig.thing_get(person_id, id as i32).await
}

async fn thing_patch(
    thing_id: u32,
    (mut pig, person_id): (Pig, i32),
    body: HashMap<String, String>,
) -> Result<impl Reply, Rejection> {
    check_id(thing_id)?;
    if !body.contains_key("name") {
        Err(custom_err(412, "missing name"))?;
    }
    pig.thing_patch(person_id, thing_id as i32, &body["name"])
        .await
}

async fn thing_delete(
    thing_id: u32,
    (mut pig, person_id): (Pig, i32),
) -> Result<impl Reply, Rejection> {
    check_id(thing_id)?;
    pig.thing_delete(person_id, thing_id as i32).await
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let auth = warp::header::<String>("apikey")
        .or_else(|_| async move { Err(custom_err(401, "needs apikey header").into()) })
        .and_then(auth);
    let form_body = warp::body::form();

    // GET /
    let people_get = warp::get()
        .and(warp::path::end())
        .and(auth)
        .and_then(people_get);

    // GET /things
    let things_get = warp::get()
        .and(warp::path("things"))
        .and(auth)
        .and_then(things_get);

    // POST /things
    let things_post = warp::post()
        .and(warp::path("things"))
        .and(auth)
        .and(form_body)
        .and_then(things_post);

    // PATCH /person
    let person_patch = warp::patch()
        .and(warp::path("person"))
        .and(auth)
        .and(form_body)
        .and_then(person_patch);

    // GET /person/<id>
    let person_get = warp::get()
        .and(path!("person" / u32))
        .and(auth)
        .and_then(person_get);

    // GET /thing/<id>
    let thing_get = warp::get()
        .and(path!("thing" / u32))
        .and(auth)
        .and_then(thing_get);

    // PATCH /thing/<id>
    let thing_patch = warp::patch()
        .and(path!("thing" / u32))
        .and(auth)
        .and(form_body)
        .and_then(thing_patch);

    // DELETE /thing/<id>
    let thing_delete = warp::delete()
        .and(path!("thing" / u32))
        .and(auth)
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
