use log::{error, info};
use reqwest::{Error, Response, StatusCode};
use crate::solana::helper::fetch;
use crate::solana::solana_beach_types::{Block, Transaction};


static BASE_URL: &str = "https://solanabeach.solanabeach.io/v1/";


pub async fn latest_transactions(limit: u32) -> Option<Vec<Transaction>> {
    let  data = fetch(format!("{}latest-transactions?limit={}", BASE_URL, limit)).await.unwrap();
    match data.status() {
        StatusCode::OK => {
            let data = serde_json::from_str::<Vec<Transaction>>(data.text().await.unwrap().as_str());
            info!("{:?}",data);
            Some(data.unwrap())
        },
        s => {error!("{:?}",s); None }
    }
}

pub async fn latest_blocks(limit: u32) -> Option<Vec<Block>> {
    let  data = fetch(format!("{}latest-blocks?limit={}", BASE_URL, limit)).await.unwrap();
    match data.status() {
        StatusCode::OK => {
            let data = serde_json::from_str::<Vec<Block>>(data.text().await.unwrap().as_str());
            info!("{:?}",data);
            Some(data.unwrap())
        },
        s => {error!("{:?}",s); None }
    }
}