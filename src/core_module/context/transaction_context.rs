
use alloy_primitives::B256;
use primitive_types::{H256, U256};
pub use serde::Deserialize;
pub use serde::Serialize;
use crate::core_module::memory::Memory;

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
pub(crate) enum StateTracerType {
    None,
    TurnOffDiff,
    TurnOnDiffPre,
    TurnOnDiffPost,
}
