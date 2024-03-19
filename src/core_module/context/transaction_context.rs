
use alloy_primitives::B256;
use ethers::prelude::{Http, Provider, TxHash};
use primitive_types::{H256, U256};
pub use serde::Serialize;
use crate::core_module::memory::Memory;
use crate::core_module::utils::bytes::pad_left;
use ethers::providers::{Middleware, ProviderError, ProviderExt};

#[derive(Debug, Clone)]
pub struct TransactionEnv {
    /// The transaction's hash
    pub tx_hash: H256,

    /// The transaction's nonce
    pub nonce: [u8; 32],

    /// Block hash. None when pending.
    pub block_hash: Option<[u8; 32]>,

    pub block_number: Option<[u8; 32]>,

    pub coinbase: Option<[u8; 20]>,

    pub timestamp: Option<[u8; 32]>,

    pub from: [u8; 20],

    pub to: Option<[u8; 20]>,

    /// Transferred value
    pub value: [u8; 32],

    pub gas_price: Option<[u8; 32]>,

    /// Gas amount
    pub gas: [u8; 32],

    /// Input data
    pub calldata: Memory,

    pub basefee: Option<[u8; 32]>,

    pub difficulty: Option<[u8; 32]>,

    pub prevrandao: Option<B256>,

    pub chain_id: Option<U256>,
}

#[derive(Debug, Clone)]
pub enum StateTracerType {
    None,
    TurnOffDiff,
    TurnOnDiffPre,
    TurnOnDiffPost,
    TXAfterState
}



pub async fn get_transaction_content(
    provider: Provider<Http>,
    tx_hash: TxHash,
) -> Result<TransactionEnv, ProviderError> {
    let transaction = provider
        .get_transaction(tx_hash)
        .await
        .expect("get transaction hash error");

    let calldata = transaction.clone().unwrap().input.to_vec();

    let block_number = transaction.clone().unwrap().block_number.unwrap();
    let block_info = provider
        .get_block(block_number)
        .await
        .expect("get block error");

    // get transaction nonce
    let mut nonce = [0u8; 32];
    transaction.clone().unwrap().nonce.to_big_endian(&mut nonce);

    // get transaction block_number[u8; 32]
    let mut _block_number = [0u8; 8];

    // transaction.clone().unwrap().block_number.unwrap().
    transaction
        .clone()
        .unwrap()
        .block_number
        .unwrap()
        .to_big_endian(&mut _block_number);
    let block_number = pad_left(&_block_number);

    // get transaction to [u8; 32]
    let to = if let Some(to) = transaction.clone().unwrap().to {
        to.0
    } else {
        [0u8; 20]
    };

    // get transaction value
    let mut value = [0u8; 32];
    transaction.clone().unwrap().value.to_big_endian(&mut value);

    // get transaction gas_price
    let mut gas_price = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .gas_price
        .unwrap()
        .to_big_endian(&mut gas_price);

    // get transaction gas
    let mut gas = [0u8; 32];
    transaction.clone().unwrap().gas.to_big_endian(&mut gas);

    // get transaction basefee
    let mut basefee = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .max_fee_per_gas
        .unwrap()
        .to_big_endian(&mut basefee);

    // get transaction difficulty
    let mut difficulty = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .difficulty
        .to_big_endian(&mut difficulty);

    // get transaction timestamp
    let mut timestamp = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .timestamp
        .to_big_endian(&mut timestamp);

    Ok(TransactionEnv {
        tx_hash: transaction.clone().unwrap().hash,
        nonce: nonce,
        block_hash: Some(transaction.clone().unwrap().block_hash.unwrap().0),
        block_number: Some(block_number),
        coinbase: Some(block_info.clone().unwrap().author.unwrap().0),
        timestamp: Some(timestamp),
        from: transaction.clone().unwrap().from.0,
        to: Some(to),
        value: value,
        gas_price: Some(gas_price),
        gas: gas,
        calldata: Memory::new(Some(calldata)),
        basefee: Some(basefee),
        difficulty: Some(difficulty),
        prevrandao: None,
        chain_id: transaction.clone().unwrap().chain_id,
    })
}



