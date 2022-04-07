use std::env;

use log::info;
use reqwest::{Error, Response};

pub async fn fetch(url: String) -> Result<Response, Error> {
    info!("Fetching: {:?}",url);
    let client = reqwest::Client::new();
    client.get(url.clone())
        .header("User-Agent", "Mozilla/5.0")
        //.bearer_auth(env::var("SOLANABEACH_TOKEN").unwrap())
        .send()
        .await
}