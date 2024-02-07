use primitive_types::U256;
use alloy_primitives::B256;

#[derive(Debug)]
pub struct EvmContext {
    pub blockhash: Option<[u8; 32]>,

    pub block_number: Option<[u8; 32]>,

    pub coinbase: Option<[u8; 20]>,

    pub timestamp: Option<[u8; 32]>,

    pub gas_price: Option<[u8; 32]>,

    pub gas_limit: Option<[u8; 32]>,

    pub basefee: Option<[u8; 32]>,

    /// The difficulty of the block.
    ///
    /// Unused after the Paris (AKA the merge) upgrade, and replaced by `prevrandao`.
    pub difficulty: Option<[u8; 32]>,
    /// The output of the randomness beacon provided by the beacon chain.
    ///
    /// Replaces `difficulty` after the Paris (AKA the merge) upgrade with [EIP-4399].
    ///
    /// NOTE: `prevrandao` can be found in a block in place of `mix_hash`.
    ///
    /// [EIP-4399]: https://eips.ethereum.org/EIPS/eip-4399
    pub prevrandao: Option<B256>,
}

impl EvmContext {
    pub fn new() -> Self {
        Self {
            blockhash: None,
            block_number: None,
            coinbase: None,
            timestamp: None,
            gas_price: None,
            gas_limit: None,
            basefee: None,
            difficulty: None,
            prevrandao: None,
        }
    }
}