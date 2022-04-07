use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Transaction {
    #[serde(rename = "transactionHash")]
    transaction_hash: String,
    #[serde(rename = "blockNumber")]
    block_number: i64,
    index: i64,
    #[serde(rename = "blocktime")]
    pub block_time: BlockTime,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Block {
    #[serde(rename = "blocknumber")]
    block_number: i64,
    #[serde(rename = "blocktime")]
    pub block_time: BlockTime,
    pub metrics: Metrics,
}


#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct BlockTime {
    pub absolute: i32,
    relative: i32,
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
pub struct Metrics {
    #[serde(rename = "txcount")]
    pub tx_count: i32,
    #[serde(rename = "failedtxs")]
    pub failed_txs: i32,
    #[serde(rename = "totalfees")]
    pub total_fees: i32,
    #[serde(rename = "instructions")]
    pub instructions: i32,
    #[serde(rename = "sucessfultxs")]
    pub successful_txs: i32,
    #[serde(rename = "innerinstructions")]
    pub inner_instructions: i32,
}


