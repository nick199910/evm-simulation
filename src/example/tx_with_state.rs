use crate::core_module::context::account_state_ex_context::{get_accounts_state_tx, ISDiff};
use crate::core_module::context::evm_context::EvmContext;
use crate::core_module::context::transaction_context::get_transaction_content;
use crate::core_module::runner::Runner;
use crate::core_module::state::EvmState;
use crate::core_module::utils::bytes::to_h256;
use dotenv::dotenv;
use ethers::addressbook::Address;
use ethers::prelude::Provider;
use ethers::providers::{Middleware, ProviderError, ProviderExt};
use ethers::types::TxHash;
pub use serde::Deserialize;
pub use serde::Serialize;
use std::env;
use std::str::FromStr;
use std::sync::Arc;

#[tokio::test]
async fn set_evm_pre_tx_state() -> Result<(), ProviderError> {
    // 1. set provider
    dotenv().ok().expect(".env file not exit");
    let provider_http_url = env::var("mainnet").unwrap_or_else(|_| {
        String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b")
    });

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");

    // let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";

    // let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    // let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e";
    // let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    // let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5";
    let HeavenGate = "0xe28ca1f43036f4768776805fb50906f8172f75eba3bf1d9866bcd64361fda834";

    // 2. Obtain the pre_transaction_account_state
    let accounts_state_pre_tx = get_accounts_state_tx(
        Arc::new(provider.clone()),
        to_h256(HeavenGate),
        ISDiff::default(),
    )
    .await;

    // 3. Obtain the transaction context
    let transaction_content =
        get_transaction_content(provider, TxHash::from_str(HeavenGate).unwrap())
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
    let mut interpreter = Runner::new(
        caller,
        Some(origin),
        Some(address),
        Some(value),
        Some(data),
        Some(state),
        None,
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

    // 9.
    let bytecode = accounts_state_pre_tx
        .get(&Address::from_slice(&transaction_content.to.unwrap()))
        .unwrap()
        .code
        .as_ref()
        .unwrap();

    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");

        // Interpret the bytecode
        let ret = interpreter.interpret(bytecode, true);
        if ret.is_ok() {
            println!("successful!!!!");
            println!("op_codelist is : {:?}",interpreter.op_list);
        }
    }

    Ok(())
}
