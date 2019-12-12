use tokio::prelude::*;

use crate::error::ErrStr;

pub struct Pig {
    client: tokio_postgres::Client,
}

impl Pig {
    pub async fn new() -> Result<Pig, Box<str>> {
        let (mut client, connection) =
            tokio_postgres::connect("host=localhost user=pig password=pig dbname=pig", tokio_postgres::NoTls)
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

    pub async fn apikey_get(&mut self, key: &str) -> Result<(), Box<str>>{
        let row = self.client
            .query_one("SELECT status, js FROM apikey_get($1)", &[&key])
            .await
            .err_str("apikey_get error")?;
        let status: i16 = row.get("status");
        let json: serde_json::Value = row.get("js");
        println!("{} {}", status, json);
        Ok(())
    }
}

