use reqwest::Client;
use serde_json::{json, Value};
use std::fs;
use ethers::core::k256::sha2::digest::typenum::op;

// 输入函数名和参数位置 得到源参数值
fn extract_origin_calldata_value(func_name_str: &str, index: u8) -> [u8; 32] {
    unimplemented!()
}

// get_op_code_list ["PUSH1", "PUSH1", "MSTORE", "PUSH1", "CALLDATASIZE", "LT"]
async fn get_opcode_list(_rpc: &str, _attack_hash: &str) -> Vec<String> {
    unimplemented!()
}

#[tokio::test]
async fn test_get_opcode_list() {


}
