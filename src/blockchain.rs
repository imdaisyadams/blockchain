extern crate time;
extern crate serde;
extern crate serde_json;
extern crate sha2;

use serde_derive::Serialize;
use sha2::{Digest, Sha256};
use std::fmt::Write;


#[derive(Debug, Clone, Serialize)]
struct Transaction {
    sender: String,
    receiver: String,
    amount: f32,
}

#[derive(Debug, Serialize)]
pub struct BlockHeader {
    timestamp: i64,
    nonce: u32,
    pre_hash: String,
    merkle: String,
    difficulty: u32,
}

#[derive(Debug, Serialize)]
pub struct Block {
    header: BlockHeader,
    count: u32,
    transaction: Vec<Transaction>,
}

pub struct Chain {
    chain: Vec<Block>,
    curr_trans: Vec<Transaction>,
    difficulty: u32,
    miner_addr: String,
    reward: f32,
}