use std::collections::HashMap;
use crate::core_module::context::transaction_context::StateTracerType;

#[derive(Debug, Clone)]
pub struct AccountStateEx {
    /// The account's nonce, which is incremented each time a transaction is sent from the account.
    pub nonce: u64,

    /// The account's balance, represented as a 32-byte array.
    pub balance: [u8; 32],

    /// The account's storage, represented as a hashmap where the keys and values are both 32-byte arrays.
    pub storage: Option<HashMap<[u8; 32], [u8; 32]>>,

    /// The hash of the account's code, represented as a 32-byte array.
    pub code_hash: Option<[u8; 32]>,

    pub code: Option<String>,

    pub state_tracer_type: StateTracerType,
}