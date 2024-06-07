use reqwest::Client;
use serde_json::{json, Value};

fn extract_op_values(logs: &Value) -> Vec<String> {
    let mut op_values = Vec::new();

    if let Some(logs_array) = logs.as_array() {
        for log in logs_array {
            if let Some(op_value) = log["op"].as_str() {
                op_values.push(op_value.to_string());
            }
        }
    }
    op_values
}

// get_op_code_list ["PUSH1", "PUSH1", "MSTORE", "PUSH1", "CALLDATASIZE", "LT"]
pub async fn get_opcode_list(_rpc: &str, _attack_hash: &str) -> Vec<String> {
    let client = Client::new();

    let res = client
        .post(_rpc)
        .json(&json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "debug_traceTransaction",
            "params": [
                _attack_hash,
                {
                    "enableMemory": false,
                    "disableStack": true,
                    "disableStorage": true,
                    "enableReturnData": false
                }
            ]
        }))
        .send()
        .await
        .expect("rpc error");
    let tracer_data = res.json::<Value>().await.expect("json lib error");

    let mut opcode_list: Vec<String> = Vec::new();
    if tracer_data["result"]["failed"].eq(&true) {
        return opcode_list;
    }

    // 获取 result 下的 structLogs 字段
    let struct_logs = tracer_data["result"]["structLogs"].clone();
    extract_op_values(&struct_logs)
}

// #[tokio::test]
async fn test_get_opcode_list() {
    let rpc = "https://blue-quiet-surf.quiknode.pro/ac44441c600862066c752e9d83b7aefb0532f03b/";
    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    let op_list = get_opcode_list(rpc, attack_hash).await;
    println!("{:?}", op_list.len())
}
