use std::env;

use log::info;
use reqwest::{Error, Response};

pub async fn fetch(url: String) -> Result<Response, Error> {
    info!("Fetching: {:?}",url);
    let client = reqwest::Client::new();
    match  env::var("SOLANABEACH_TOKEN") {
        Ok(val) => {client.get(url.clone())
            .header("User-Agent", "Mozilla/5.0")
            .bearer_auth(val)
            .send()
            .await},
        _ => {
            client.get(url.clone())
                .header("User-Agent", "Mozilla/5.0")
                .send()
                .await
        }
    }

}