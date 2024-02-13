use ethers::prelude::{GethDebugTracingOptions, Http, Provider};
use ethers::providers::{Middleware, ProviderExt};
use std::process;
use std::sync::Arc;

use crate::core_module::utils::bytes::{pad_left, to_h256};
use primitive_types::H256;

async fn get_accounts_state_pre_tx(provider: Arc<Provider<Http>>, tx_hash: H256) {
    // let tracer = GethDebugTracerType::BuiltInTracer()
    let mut options = GethDebugTracingOptions::default();

    // options.disable_stack = Some(true);
    // options.disable_storage = Some(false);
    // options.enable_memory = Some(false);
    // options.enable_return_data = Some(true);
    // options.tracer = Some(GethDebugTracerType::JsTracer(String::from("{data: [], fault: function(log) {}, step: function(log) { if(log.op.toString() == \"CALL\") this.data.push(log.stack.peek(0)); }, result: function() { return this.data; }}")));

    let mut tracer_info = provider
        .debug_trace_transaction(tx_hash, options)
        .await
        .unwrap_or_else(|err| {
            eprintln!("transaction reverted with err: {}", err);
            process::exit(1);
        });

    println!("{:?}", tracer_info);
}

#[tokio::test]
async fn test_get_accounts_state_pre_tx() {
    let provider_http_url = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let provider = Provider::try_connect(provider_http_url)
        .await
        .expect("rpc connect error");

    let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e"; // 可行
    let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf"; // 可行
    let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5"; // 可行

    get_accounts_state_pre_tx(Arc::new(provider.clone()), to_h256(uniswap_v2_attack)).await;
}
