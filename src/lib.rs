use block::Block;
use chrono::Utc;
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

mod block;
mod web;

#[wasm_bindgen]
pub fn mine() -> Result<(), JsValue> {
    let mut block = Block::new(Utc::now().timestamp(), vec![], &Block::first())
        .expect("failed to create block");
    block
        .mine("0000ffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff")
        .expect("failed to mine block");
    web::add_heading_block(
        block.nonce.to_string(),
        block.hash().expect("failed to hash block"),
    )?;

    Ok(())
}
