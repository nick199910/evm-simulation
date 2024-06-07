use ethers::core::k256::sha2::digest::typenum::op;
use ethers::types::GethDebugTracerConfig;
use ethers::{
    core::types::{GethDebugTracingOptions, H256},
    providers::{Http, Middleware, Provider},
};
use primitive_types::U256;
use serde::Deserialize;
use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io::Read;
use std::str::FromStr;

/// use `debug_traceTransaction` to fetch traces
/// requires, a valid endpoint in `RPC_URL` env var that supports `debug_traceTransaction`
///
#[derive(Debug)]
pub struct OpCodeContext {
    pub pc: u64,

    pub op_code: String,

    pub stack: Option<Vec<U256>>,

    pub memory: Option<Vec<String>>,

    pub storage: Option<BTreeMap<H256, H256>>,
}

#[tokio::test]
async fn test_call_tracer() -> () {
    let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e";
    let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5";

    let url = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let client = Provider::<Http>::try_from(url).unwrap();
    let tx_hash = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let h: H256 = H256::from_str(uniswap_v2_attack).unwrap();

    // default tracer
    let options = GethDebugTracingOptions::default();

    let traces = client.debug_trace_transaction(h, options).await.unwrap();

    if let ethers::types::GethTrace::Known(ethers::types::GethTraceFrame::Default(tracer_info)) =
        traces
    {
        // println!("{:?}", tracer_info);
        let tracer_list: Vec<OpCodeContext> = tracer_info
            .struct_logs
            .into_iter()
            .map(|step| OpCodeContext {
                pc: step.pc,
                op_code: step.op,
                memory: step.memory,
                stack: step.stack,
                storage: step.storage,
            })
            .collect();

        println!("{:?}", tracer_list);

        // gas: tracer_info.gas.as_u64(),
        // failed: anvil_trace.failed,
        // return_value: hex::encode(anvil_trace.return_value.as_ref()), // TODO see if 0x adjustment is needed
        // struct_logs: anvil_trace
        //     .struct_logs
        //     .into_iter()
        //     .map(|step|
    }
}

#[derive(Debug, Deserialize)]
pub struct StructLog {
    pub pc: u32,
    pub op: String,
    pub gas: u32,
    pub gasCost: u32,
    pub depth: u32,
    pub stack: Vec<String>,
    pub memory: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct LogData {
    pub structLogs: Vec<StructLog>,
}

pub fn read_op_tracer() -> LogData {
    let mut file =
        File::open("tracer_data/euler_attack_tx_op_logs.json").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Unable to read the file");

    // 解析JSON数据
    let log_data: LogData = serde_json::from_str(&contents).expect("Unable to parse JSON");
    log_data
}

// #[test]
fn test_reuiad_op_tracer() {
    // 读取JSON文件
    let _ = read_op_tracer();
}
