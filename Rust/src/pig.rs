use tokio::prelude::*;

use crate::error::*;

use warp::http::StatusCode;

pub struct Pig {
    client: tokio_postgres::Client,
}

impl Pig {
    pub async fn new() -> Result<Pig, Error> {
        let (client, connection) = tokio_postgres::connect(
            "host=localhost user=pig password=pig dbname=pig",
            tokio_postgres::NoTls,
        )
        .await
        .err_str("Can't connect DB")?;

        let connection = connection.map(|r| {
            if let Err(e) = r {
                eprintln!("connection error: {}", e);
            }
        });
        tokio::spawn(connection);
        Ok(Pig { client })
    }

    pub async fn apikey_get(&mut self, key: &str) -> Result<Option<i32>, Error> {
        let row = self
            .client
            .query_one("SELECT status, js FROM apikey_get($1)", &[&key])
            .await
            .err_str("apikey_get error")?;
        let status: i16 = row.get("status");
        if status == 200 {
            #[derive(Deserialize)]
            struct Temp {
                person_id: i32,
            }
            let json: serde_json::Value = row.get("js");
            let json: Temp = serde_json::from_value(json).err_str("can't parse json")?;
            Ok(Some(json.person_id))
        } else {
            Ok(None)
        }
    }

    pub async fn people_get(&mut self) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one("SELECT status, js FROM people_get()", &[])
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn things_get(&mut self, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one("SELECT status, js FROM things_get($1)", &[&id])
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn thing_get(
        &mut self,
        id: i32,
        thing_id: i32,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one(
                "SELECT status, js FROM thing_get($1, $2)",
                &[&id, &thing_id],
            )
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn thing_add(
        &mut self,
        id: i32,
        thing: &str,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one("SELECT status, js FROM thing_add($1, $2)", &[&id, &thing])
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn thing_delete(
        &mut self,
        person_id: i32,
        thing_id: i32,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one(
                "SELECT status, js FROM thing_delete($1, $2)",
                &[&person_id, &thing_id],
            )
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn person_patch(
        &mut self,
        id: i32,
        name: &str,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one(
                "SELECT status, js FROM person_update($1, $2)",
                &[&id, &name],
            )
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn thing_patch(
        &mut self,
        person_id: i32,
        thing_id: i32,
        name: &str,
    ) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one(
                "SELECT status, js FROM thing_update($1, $2, $3)",
                &[&person_id, &thing_id, &name],
            )
            .await
            .err_str("rip")?;
        process_result(row)
    }

    pub async fn person_get(&mut self, id: i32) -> Result<impl warp::Reply, warp::Rejection> {
        let row = self
            .client
            .query_one("SELECT status, js FROM person_get($1)", &[&id])
            .await
            .err_str("rip")?;
        process_result(row)
    }
}

fn process_result(row: tokio_postgres::Row) -> Result<impl warp::Reply, warp::Rejection> {
    use std::convert::TryInto;
    let status: i16 = row.get("status");
    let json: serde_json::Value = row.get("js");
    let json = warp::reply::json(&json);
    Ok(warp::reply::with_status(
        json,
        StatusCode::from_u16(status.try_into().unwrap()).expect("DB return invalid HTTP code"),
    ))
}
