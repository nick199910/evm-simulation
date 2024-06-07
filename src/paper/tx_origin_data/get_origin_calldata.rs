use reqwest::Client;
use serde_json::{json, Value};
use std::collections::HashSet;

use crate::bytes::_hex_string_to_bytes;

// 将call_tracer里面的信息全部读取到一个list里面
fn recursive_read_input(data: &Value) -> Vec<String> {
    let mut ret_calldata = Vec::new();
    if let Some(data_map) = data.as_object() {
        if let Some(input_data) = data_map.get("input") {
            if let Some(input_str) = input_data.as_str() {
                ret_calldata.push(input_str.to_string());
            }
        }
        for value in data_map.values() {
            ret_calldata.extend(recursive_read_input(value));
        }
    } else if let Some(data_list) = data.as_array() {
        for item in data_list {
            ret_calldata.extend(recursive_read_input(item));
        }
    }
    ret_calldata
}

fn func_name_to_selector(func_name_str: &str) -> String {
    let selector = &ethers::core::utils::keccak256(func_name_str)[0..4];
    // 将十进制数组转换为十六进制字符串
    let selector_str: String = [
        "0x",
        &selector
            .iter()
            .map(|&x| format!("{:02X}", x).to_lowercase())
            .collect::<Vec<String>>()
            .join(""),
    ]
    .concat();

    println!("{:?}", selector_str);
    selector_str
}

// func_name_str ""
async fn get_origin_calldata(
    _rpc: &str,
    _attack_hash: &str,
    func_name_str: &str,
    _index: u8,
) -> Vec<u8> {
    // 拿到 call_tracer
    let client = Client::new();

    let res = client
        .post(_rpc)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "debug_traceTransaction",
            "params": [
                _attack_hash,
                {"tracer": "callTracer"}
            ]
        }))
        .send()
        .await
        .expect("rpc error");
    let tracer_data = res.json::<Value>().await.expect("json lib error");

    let mut param_data: Vec<u8> = Vec::new();
    if tracer_data["result"]["failed"].eq(&true) {
        return param_data;
    }

    let call_data = tracer_data["result"]["calls"].clone();

    let input_data_list = recursive_read_input(&call_data);
    println!("{:?}", input_data_list);

    let func_selector = func_name_to_selector(func_name_str);

    // 过滤并去重
    let mut unique_calldata_param = HashSet::new();
    for item in input_data_list {
        if item.starts_with(func_selector.as_str()) {
            unique_calldata_param.insert(item);
        }
    }

    // 将 HashSet 转换回 Vec
    let origin_calldata_param: Vec<String> = unique_calldata_param.into_iter().collect();

    // 计算起始位置和结束位置 0x + 4_bytes_function_selector
    let start_position = (2 + 8 + (_index - 1) * 64) as usize;
    let end_position = start_position + 64;

    // 截取子字符串
    let index_param_value = &origin_calldata_param[0][start_position..end_position];
    let index_param_value_bytes = _hex_string_to_bytes(index_param_value);

    index_param_value_bytes
}

// #[tokio::test]
async fn test_get_opcode_list() {
    let rpc = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    let origin_param_data =
        get_origin_calldata(rpc, attack_hash, "redeem(address,uint256)", 2).await;
}
