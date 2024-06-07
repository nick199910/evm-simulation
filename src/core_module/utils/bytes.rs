use ethers::types::U256;
use std::str::FromStr;

// Colored output
use crate::core_module::op_codes;
use colored::*;
use primitive_types::{H160, H256};

/// Pad a [u8] with no particular length to 32 bytes to return a [u8; 32]
/// It adds zero padding to the left
///
/// # Arguments
///
/// * `bytes` - The [u8] to pad
///
/// # Returns
///
/// Returns a [u8; 32] with the padded bytes
pub fn pad_left(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

//
pub fn pad_left_one(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [255u8; 32];
    padded[32 - bytes.len()..].copy_from_slice(bytes);
    padded
}

/// Pad a [u8] with no particular length to 32 bytes to return a [u8; 32]
/// It adds zeros to the right of the [u8] instead of the left
///
/// # Arguments
///
/// * `bytes` - The [u8] to pad
///
/// # Returns
///
/// Returns a [u8; 32] with the padded bytes
pub fn _pad_right(bytes: &[u8]) -> [u8; 32] {
    let mut padded = [0u8; 32];
    padded[..bytes.len()].copy_from_slice(bytes);
    padded
}

/// Convert a [u8; 32] to a [u8; 20]
///
/// # Arguments
///
/// * `bytes` - The [u8; 32] to convert
///
/// # Returns
///
/// Returns a [u8; 20] with the address
pub fn bytes32_to_address(bytes: &[u8; 32]) -> [u8; 20] {
    let mut address = [0u8; 20];
    address.copy_from_slice(&bytes[12..]);
    address
}

/// Remove zero padding from a [u8; 32] to return only the relevant bytes
///
/// # Arguments
///
/// * `arr` - The [u8; 32] to remove zero padding from
///
/// # Returns
///
/// Returns a [u8] with the zero padding removed
pub fn strip_zero_padding(arr: &[u8; 32]) -> &[u8] {
    let start = arr.iter().position(|&x| x != 0).unwrap_or(0);
    let end = arr.iter().rposition(|&x| x != 0).unwrap_or(0) + 1;
    &arr[start..end]
}

/// Convert a u64 to a [u8; 32]
///
/// # Arguments
///
/// * `n` - The u64 to convert
///
/// # Returns
///
/// Returns a [u8; 32] with the u64 converted to bytes
pub fn u64_to_u256_array(n: u64) -> [u8; 32] {
    let uint256 = U256::from(n);
    let mut bytes = [0u8; 32];
    uint256.to_big_endian(&mut bytes);
    bytes
}

pub fn u64_x4_array_to_u8_x32_array(arr: U256) -> [u8; 32] {
    let mut arr_u8 = [0u8; 32];
    arr.to_big_endian(&mut arr_u8);
    arr_u8
}

pub fn _hex_string_to_bytes(hex: &str) -> Vec<u8> {
    let mut after_hex = hex;
    if hex.starts_with("0x") {
        after_hex = &hex[2..];
    }
    match hex::decode(after_hex) {
        Ok(bytes) => bytes,
        Err(e) => {
            panic!("Error: {}", e.to_string().red());
        }
    }
}

#[test]
pub fn test__hex_string_to_bytes() {
    let tt = "0x60806040526004361061006d576000357c0100000000000000000000000000000000000000000000000000000000900463ffffffff1680633659cfe6146100775780634f1ef286146100ba5780635c60da1b146101085780638f2839701461015f578063f851a440146101a2575b6100756101f9565b005b34801561008357600080fd5b506100b8600480360381019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610213565b005b610106600480360381019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190803590602001908201803590602001919091929391929390505050610268565b005b34801561011457600080fd5b5061011d610308565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b34801561016b57600080fd5b506101a0600480360381019080803573ffffffffffffffffffffffffffffffffffffffff169060200190929190505050610360565b005b3480156101ae57600080fd5b506101b761051e565b604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390f35b610201610576565b61021161020c610651565b610682565b565b61021b6106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141561025c57610257816106d9565b610265565b6102646101f9565b5b50565b6102706106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614156102fa576102ac836106d9565b3073ffffffffffffffffffffffffffffffffffffffff163483836040518083838082843782019150509250505060006040518083038185875af19250505015156102f557600080fd5b610303565b6103026101f9565b5b505050565b60006103126106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614156103545761034d610651565b905061035d565b61035c6101f9565b5b90565b6103686106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141561051257600073ffffffffffffffffffffffffffffffffffffffff168173ffffffffffffffffffffffffffffffffffffffff1614151515610466576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260368152602001807f43616e6e6f74206368616e6765207468652061646d696e206f6620612070726f81526020017f787920746f20746865207a65726f20616464726573730000000000000000000081525060400191505060405180910390fd5b7f7e644d79422f17c01e4894b5f4f588d331ebfa28653d42ae832dc59e38c9798f61048f6106a8565b82604051808373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020018273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019250505060405180910390a161050d81610748565b61051b565b61051a6101f9565b5b50565b60006105286106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff16141561056a576105636106a8565b9050610573565b6105726101f9565b5b90565b61057e6106a8565b73ffffffffffffffffffffffffffffffffffffffff163373ffffffffffffffffffffffffffffffffffffffff1614151515610647576040517f08c379a00000000000000000000000000000000000000000000000000000000081526004018080602001828103825260328152602001807f43616e6e6f742063616c6c2066616c6c6261636b2066756e6374696f6e20667281526020017f6f6d207468652070726f78792061646d696e000000000000000000000000000081525060400191505060405180910390fd5b61064f610777565b565b6000807f7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c36001029050805491505090565b3660008037600080366000845af43d6000803e80600081146106a3573d6000f35b3d6000fd5b6000807f10d6a54a4754c8869d6886b5f5d7fbfa5b4522237ea5c60d11bc4e7a1ff9390b6001029050805491505090565b6106e281610779565b7fbc7cd75a20ee27fd9adebab32041f755214dbc6bffa90cc0225b39da2e5c2d3b81604051808273ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200191505060405180910390a150565b60007f10d6a54a4754c8869d6886b5f5d7fbfa5b4522237ea5c60d11bc4e7a1ff9390b60010290508181555050565b565b60006107848261084b565b151561081e576040517f08c379a000000000000000000000000000000000000000000000000000000000815260040180806020018281038252603b8152602001807f43616e6e6f742073657420612070726f787920696d706c656d656e746174696f81526020017f6e20746f2061206e6f6e2d636f6e74726163742061646472657373000000000081525060400191505060405180910390fd5b7f7050c9e0f4ca769c69bd3a8ef740bc37934f8e2c036e5a723fd8ee048ed3f8c360010290508181555050565b600080823b9050600081119150509190505600a165627a7a72305820a4a547cfc7202c5acaaae74d428e988bc62ad5024eb0165532d3a8f91db4ed240029";
    _hex_string_to_bytes(tt);
}

/* -------------------------------------------------------------------------- */
/*                               Math operations                              */
/* -------------------------------------------------------------------------- */

/// Add a u64 to a u256 expressed as a [u8; 32]
///
/// # Arguments
///
/// * `arr` - The [u8; 32] to add to
///
/// * `number` - The u64 to add
///
/// # Returns
///
/// Returns a [u8; 32] with the u64 added to the [u8; 32]
pub fn _add(arr: [u8; 32], number: u64) -> [u8; 32] {
    // Convert the [u8; 32] into U256
    let num = U256::from_big_endian(&arr);

    // Add
    let num = num + U256::from(number);

    // Convert back to [u8; 32]
    let mut result = [0u8; 32];
    num.to_big_endian(&mut result);

    result
}

pub fn to_h160(str_address: &'static str) -> H160 {
    H160::from_str(str_address).unwrap()
}

pub fn to_h256(str_address: &'static str) -> H256 {
    H256::from_str(str_address).unwrap()
}
