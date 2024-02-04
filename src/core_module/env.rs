use primitive_types::U256;
use alloy_primitives::B256;

#[derive(Debug)]
pub struct BlockEnv {
    /// The number of ancestor blocks of this block (block height).
    pub number: [u8; 32],
    /// Coinbase or miner or address that created and signed the block.
    ///
    /// This is the receiver address of all the gas spent in the block.
    pub coinbase: [u8; 20],

    /// The timestamp of the block in seconds since the UNIX epoch.
    pub timestamp: [u8; 32],
    /// The gas limit of the block.
    pub gas_limit: U256,
    /// The base fee per gas, added in the London upgrade with [EIP-1559].
    ///
    /// [EIP-1559]: https://eips.ethereum.org/EIPS/eip-1559
    pub basefee: U256,
    /// The difficulty of the block.
    ///
    /// Unused after the Paris (AKA the merge) upgrade, and replaced by `prevrandao`.
    pub difficulty: U256,
    /// The output of the randomness beacon provided by the beacon chain.
    ///
    /// Replaces `difficulty` after the Paris (AKA the merge) upgrade with [EIP-4399].
    ///
    /// NOTE: `prevrandao` can be found in a block in place of `mix_hash`.
    ///
    /// [EIP-4399]: https://eips.ethereum.org/EIPS/eip-4399
    pub prevrandao: Option<B256>,

}
#[derive(Debug)]
pub struct Env {
    /// Configuration of the block the transaction is in.
    pub block: BlockEnv,
}

#[derive(Debug)]
pub struct EvmContext {
    pub env: Env,
}