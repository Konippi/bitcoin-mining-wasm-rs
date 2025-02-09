use crypto::{digest::Digest, sha2::Sha256};
use num_bigint::BigUint;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Block {
    ts: i64,
    pub nonce: u64,
    txs: Vec<Transaction>,
    previous_hash: String,
}

#[derive(Debug, Serialize)]
pub struct Transaction {
    id: String,
    ts: u64,
    payload: String,
}

impl Block {
    pub fn new(ts: i64, txs: Vec<Transaction>, previous_block: &Self) -> anyhow::Result<Self> {
        Ok(Self {
            ts,
            nonce: 0,
            txs,
            previous_hash: previous_block.hash()?,
        })
    }

    pub fn hash(&self) -> anyhow::Result<String> {
        let mut hasher = Sha256::new();
        hasher.input_str(&self.as_json()?);

        let first_hash = hasher.result_str();
        hasher.reset();
        hasher.input_str(&first_hash);

        Ok(hasher.result_str())
    }

    pub fn as_json(&self) -> anyhow::Result<String> {
        serde_json::to_string(self).map_err(Into::into)
    }

    pub fn first() -> Self {
        let tx = Transaction {
            id: "txid".to_string(),
            payload: "somexdr".to_string(),
            ts: 0,
        };

        Self {
            ts: 0,
            nonce: 0,
            txs: vec![tx],
            previous_hash: "0".to_string(),
        }
    }

    pub fn mine(&mut self, target_hex: &str) -> anyhow::Result<()> {
        while !self.is_valid(target_hex)? {
            self.nonce += 1;
        }

        Ok(())
    }

    pub fn is_valid(&self, target_hex: &str) -> anyhow::Result<bool> {
        let hash_hex = &self.hash()?;
        let hash_num = BigUint::from_bytes_be(&hex::decode(hash_hex)?);
        let target_num = BigUint::from_bytes_be(&hex::decode(target_hex)?);

        Ok(hash_num < target_num)
    }
}
