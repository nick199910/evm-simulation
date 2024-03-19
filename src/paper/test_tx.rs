
use crate::core_module::runner::Runner;
use crate::core_module::state::EvmState;
use crate::core_module::utils::bytes::{to_h256};
use ethers::prelude::{Provider};
use ethers::providers::{Middleware, ProviderError, ProviderExt};
use ethers::types::{Address, TxHash};
pub use serde::Deserialize;
pub use serde::Serialize;
use std::str::FromStr;
use std::sync::Arc;
use crate::core_module::context::account_state_ex_context::{get_accounts_state_tx, ISDiff};
use crate::core_module::context::evm_context::EvmContext;
use crate::core_module::context::calldata_info::CallDataInfo;
use crate::core_module::context::transaction_context::{get_transaction_content};
use dotenv::dotenv;
use std::env;
use crate::bytes::_hex_string_to_bytes;

#[tokio::test]
async fn test_tx_state() -> Result<(), ProviderError> {

    // 1. set provider
    dotenv().ok().expect(".env file not exit");
    let provider_http_url = env::var("mainnet")
        .unwrap_or_else(|_| String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b"));

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");

    let olympus_dao_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    // 2. Obtain the pre_transaction_account_state, 需要把这个状态改为post的状态
    let accounts_state_pre_tx =
        get_accounts_state_tx(Arc::new(provider.clone()), to_h256(olympus_dao_tx), ISDiff::default()).await;

    // 3. Obtain the transaction context
    let transaction_content =
        get_transaction_content(provider, TxHash::from_str(olympus_dao_tx).unwrap())
            .await
            .expect("get transaction hash error");

    let state: EvmState;
    state = EvmState::new(None);

    // 4. Set the transaction context for the virtual machine
    let caller = transaction_content.from;
    let origin = transaction_content.from;
    let address = transaction_content.to.unwrap();
    let value = transaction_content.value;
    let data = transaction_content.calldata.heap;

    // 5. Create a new interpreter
    let mut interpreter = Runner::new_paper(
        caller,
        Some(origin),
        Some(address),
        Some(value),
        Some(data),
        Some(state),
        None,
        None
    );

    // 6. insert account_state to evm
    accounts_state_pre_tx
        .iter()
        .for_each(|(_addr, _account_state_ex)| {
            interpreter.modify_account_state(_addr.0, _account_state_ex.clone());
        });

    // 7. set evm state NULL env
    let mut evm_context = EvmContext::new(); // Adjust this based on your actual implementation

    // 8. update evm state env
    evm_context.gas_price = transaction_content.gas_price;
    evm_context.block_number = transaction_content.block_number;
    evm_context.basefee = transaction_content.basefee;
    evm_context.coinbase = transaction_content.coinbase;
    evm_context.blockhash = transaction_content.block_hash;
    evm_context.difficulty = transaction_content.difficulty;
    evm_context.timestamp = transaction_content.timestamp;

    interpreter.evm_context = Some(evm_context);


    // update calldata
    let mut calldata_info = CallDataInfo::new();
    let origin_data = _hex_string_to_bytes("0x00000000000000000000000000000000000000000000000000001baeaf3816f8");
    calldata_info.origin = origin_data.clone();
    println!("{:?}", calldata_info.origin);
    interpreter.calldata_info = Some(calldata_info);
    //
    // exec bytecode
    let bytecode = accounts_state_pre_tx
        .get(&Address::from_slice(&transaction_content.to.unwrap()))
        .unwrap()
        .code
        .as_ref()
        .unwrap();

    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");
        let new_param = _hex_string_to_bytes("0x00000000000000000000000000000000000000000000000000001baeaf3816f6");
        // Interpret the bytecode
        let ret = interpreter.interpret_init(bytecode,new_param, true);
        if ret.is_ok() {
            println!("{:?}", interpreter.op_list.len());
            println!("successful!!!!");
        } else {
            println!("fail!!!!!!")
        }
    }

    Ok(())
}

