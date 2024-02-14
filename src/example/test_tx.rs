use crate::core_module::memory::Memory;
use crate::core_module::runner::Runner;
use crate::core_module::state::EvmState;
use crate::core_module::utils::bytes::{pad_left, to_h256};
use alloy_primitives::B256;
use ethers::addressbook::Address;
use ethers::prelude::GethDebugBuiltInTracerType::PreStateTracer;
use ethers::prelude::{
    Bytes, GethDebugBuiltInTracerConfig, GethDebugTracerConfig, GethDebugTracerType,
    GethDebugTracingOptions, Http, PreStateConfig, PreStateFrame, Provider, U64,
};
use ethers::providers::{Middleware, ProviderError, ProviderExt};
use ethers::types::{PreStateMode, Res, TxHash};
use ethers::utils::keccak256;
use primitive_types::{H256, U256};
pub use serde::Deserialize;
pub use serde::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::mem::transmute;
use std::process;
use std::str::FromStr;
use std::sync::Arc;
use crate::core_module::context::account_state_ex_context::AccountStateEx;
use crate::core_module::context::evm_context::EvmContext;
use crate::core_module::context::transaction_context::{StateTracerType, TransactionEnv};


async fn get_transaction_content(
    provider: Provider<Http>,
    tx_hash: TxHash,
) -> Result<TransactionEnv, ProviderError> {
    let transaction = provider
        .get_transaction(tx_hash)
        .await
        .expect("get transaction hash error");

    let calldata = transaction.clone().unwrap().input.to_vec();

    let block_number = transaction.clone().unwrap().block_number.unwrap();
    let block_info = provider
        .get_block(block_number)
        .await
        .expect("get block error");

    // get transaction nonce
    let mut nonce = [0u8; 32];
    transaction.clone().unwrap().nonce.to_big_endian(&mut nonce);

    // get transaction block_number[u8; 32]
    let mut _block_number = [0u8; 8];
    // transaction.clone().unwrap().block_number.unwrap().
    transaction
        .clone()
        .unwrap()
        .block_number
        .unwrap()
        .to_big_endian(&mut _block_number);
    let block_number = pad_left(&_block_number);

    // get transaction to [u8; 32]
    let to = if let Some(to) = transaction.clone().unwrap().to {
        to.0
    } else {
        [0u8; 20]
    };

    // get transaction value
    let mut value = [0u8; 32];
    transaction.clone().unwrap().value.to_big_endian(&mut value);

    // get transaction gas_price
    let mut gas_price = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .gas_price
        .unwrap()
        .to_big_endian(&mut gas_price);

    // get transaction gas
    let mut gas = [0u8; 32];
    transaction.clone().unwrap().gas.to_big_endian(&mut gas);

    // get transaction basefee
    let mut basefee = [0u8; 32];
    transaction
        .clone()
        .unwrap()
        .max_fee_per_gas
        .unwrap()
        .to_big_endian(&mut basefee);

    // get transaction difficulty
    let mut difficulty = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .difficulty
        .to_big_endian(&mut difficulty);

    // get transaction timestamp
    let mut timestamp = [0u8; 32];
    block_info
        .clone()
        .unwrap()
        .timestamp
        .to_big_endian(&mut timestamp);

    Ok(TransactionEnv {
        tx_hash: transaction.clone().unwrap().hash,
        nonce: nonce,
        block_hash: Some(transaction.clone().unwrap().block_hash.unwrap().0),
        block_number: Some(block_number),
        coinbase: Some(block_info.clone().unwrap().author.unwrap().0),
        timestamp: Some(timestamp),
        from: transaction.clone().unwrap().from.0,
        to: Some(to),
        value: value,
        gas_price: Some(gas_price),
        gas: gas,
        calldata: Memory::new(Some(calldata)),
        basefee: Some(basefee),
        difficulty: Some(difficulty),
        prevrandao: None,
        chain_id: transaction.clone().unwrap().chain_id,
    })
}



fn get_test_account_msg() -> AccountStateEx {
    let code = "0x608060405234801561000f575f80fd5b506004361061003f575f3560e01c80633fb5c1cb14610043578063893d20e81461005f578063a6f9dae114610069575b5f80fd5b61005d6004803603810190610058919061011e565b610085565b005b61006761009a565b005b610083600480360381019061007e91906101a3565b6100a5565b005b600c8110156100975760056001819055505b50565b6100a3336100a5565b565b805f806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b5f80fd5b5f819050919050565b6100fd816100eb565b8114610107575f80fd5b50565b5f81359050610118816100f4565b92915050565b5f60208284031215610133576101326100e7565b5b5f6101408482850161010a565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61017282610149565b9050919050565b61018281610168565b811461018c575f80fd5b50565b5f8135905061019d81610179565b92915050565b5f602082840312156101b8576101b76100e7565b5b5f6101c58482850161018f565b9150509291505056fea26469706673582212205583d88608fb17547ba77bd69a991b8647738b63e2a17f2d3a0c5dfd1fa118a364736f6c63430008160033";
    AccountStateEx {
        nonce: 1,
        balance: pad_left(&[0x00]),
        storage: Default::default(),
        code_hash: Some(keccak256(code)),
        code: Some(code.to_string()),
        state_tracer_type: StateTracerType::None,
    }
}

// #[tokio::test]
async fn test_no_state_run() -> Result<(), ProviderError> {
    let sepolia_rpc = "https://rough-frosty-field.ethereum-sepolia.quiknode.pro/40fa9bf59d6007a200145efb93862af9a528e8ae/";

    let set_five_hash = "0xceb77591c14a3a8458741a0a1e205e56d319f970c6ef497a41917df464401561";
    let set_six_hash = "0x55e59ddfb18232d0e23e5b9675900ea3e614490c5e801fa75a3cf572ebf70ca2";

    let provider = Provider::try_connect(sepolia_rpc)
        .await
        .expect("rpc connect error");

    let transaction_content =
        get_transaction_content(provider, TxHash::from_str(set_six_hash).unwrap())
            .await
            .expect("get transaction hash error");
    let test_account_msg = get_test_account_msg();

    let mut caller = transaction_content.clone().from;
    let mut origin: Option<[u8; 20]> = None;
    let mut address: Option<[u8; 20]> = Some(transaction_content.to.unwrap());
    let mut value: Option<[u8; 32]> = Some(transaction_content.value);
    let mut data: Option<Vec<u8>> = Some(transaction_content.calldata.heap);
    let mut bytecode: String = test_account_msg.code.unwrap();

    let state: EvmState;

    state = EvmState::new(None);

    // Create a new interpreter
    let mut interpreter = Runner::new(caller, origin, address, value, data, Some(state), None);

    // interpreter.state.accounts.insert()

    // Check if bytecode is an hex value of a file path
    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");

        // Interpret the bytecode
        let ret = interpreter.interpret(bytecode, true);
        if ret.is_ok() {
            println!("successful!!!!")
        }
    }
    Ok(())
}

#[tokio::test]
async fn set_evm_pre_tx_state() -> Result<(), ProviderError> {
    let provider_http_url = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let provider = Provider::try_connect(provider_http_url)
        .await
        .expect("rpc connect error");

    let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e"; // 可行
    let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf"; // 可行
    let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5"; // 可行

    // Obtain the pre-transaction account state
    let accounts_state_pre_tx =
        get_accounts_state_pre_tx(Arc::new(provider.clone()), to_h256(euler_attack), false).await;

    // Obtain the transaction context
    let transaction_content =
        get_transaction_content(provider, TxHash::from_str(euler_attack).unwrap())
            .await
            .expect("get transaction hash error");

    let state: EvmState;
    state = EvmState::new(None);

    // Set the transaction context for the virtual machine
    let caller = transaction_content.from;
    let origin = transaction_content.from;
    let address = transaction_content.to.unwrap();
    let value = transaction_content.value;
    let data = transaction_content.calldata.heap;

    // Create a new interpreter
    let mut interpreter = Runner::new(
        caller,
        Some(origin),
        Some(address),
        Some(value),
        Some(data),
        Some(state),
        None,
    );

    accounts_state_pre_tx
        .iter()
        .for_each(|(_addr, _account_state_ex)| {
            interpreter.modify_account_state(_addr.0, _account_state_ex.clone());
        });

    // set evm state env
    let mut evm_context = EvmContext::new(); // Adjust this based on your actual implementation

    // Now update the fields
    evm_context.gas_price = transaction_content.gas_price;
    evm_context.block_number = transaction_content.block_number;
    evm_context.basefee = transaction_content.basefee;
    evm_context.coinbase = transaction_content.coinbase;
    evm_context.blockhash = transaction_content.block_hash;
    evm_context.difficulty = transaction_content.difficulty;
    evm_context.timestamp = transaction_content.timestamp;

    interpreter.evm_context = Some(evm_context);

    let bytecode = accounts_state_pre_tx
        .get(&Address::from_slice(&transaction_content.to.unwrap()))
        .unwrap()
        .code
        .as_ref()
        .unwrap();

    // Check if bytecode is an hex value of a file path
    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");

        // Interpret the bytecode
        let ret = interpreter.interpret(bytecode, true);
        if ret.is_ok() {
            println!("successful!!!!")
        }
    }

    Ok(())
}

async fn get_accounts_state_pre_tx(
    provider: Arc<Provider<Http>>,
    tx_hash: H256,
    is_diff: bool,
) -> BTreeMap<Address, AccountStateEx> {
    let tracer_config = GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(is_diff),
        }),
    );

    let mut options = GethDebugTracingOptions::default();
    options.tracer = Some(GethDebugTracerType::BuiltInTracer(PreStateTracer));
    options.tracer_config = Some(tracer_config);

    let tracer_info = provider
        .debug_trace_transaction(tx_hash, options)
        .await
        .unwrap_or_else(|err| {
            eprintln!("transaction reverted with err: {}", err);
            process::exit(1);
        });

    let mut account_state_ex: BTreeMap<Address, AccountStateEx> = BTreeMap::new();

    match tracer_info {
        ethers::types::GethTrace::Known(ref a) => match a {
            ethers::types::GethTraceFrame::PreStateTracer(b) => match b {
                PreStateFrame::Default(df) => {
                    let pre_state = &df.0;

                    pre_state.iter().for_each(|(_addr, _account_state)| {
                        let mut balance_u64 = _account_state.balance.clone().unwrap();
                        let mut balance_u8 = vec![0u8; 32];
                        balance_u64.to_big_endian(balance_u8.as_mut());

                        let code = _account_state.clone().code;

                        let code_hash = if let Some(inner_code) = code.clone() {
                            Some(keccak256(&inner_code))
                        } else {
                            None
                        };

                        let storage = _account_state.storage.clone();
                        let mut my_storage: HashMap<[u8; 32], [u8; 32]> = HashMap::new();
                        if let Some(inner_storage) = storage.clone() {
                            inner_storage.iter().for_each(|(slot, value)| {
                                my_storage.insert(slot.0, value.0);
                            });
                        };
                        let nonce = if let Some(inner_nonce) = _account_state.nonce {
                            inner_nonce.as_u64()
                        } else {
                            0
                        };

                        let mut account_state = AccountStateEx {
                            nonce: nonce,
                            balance: pad_left(&balance_u8),
                            storage: Some(my_storage),
                            code_hash: code_hash,
                            code: code,
                            state_tracer_type: StateTracerType::None,
                        };
                        account_state_ex.insert(*_addr, account_state.clone());
                    });
                }
                PreStateFrame::Diff(dm) => {
                    println!("{:?}", 1111);
                }
            },
            _ => todo!(),
        },
        _ => todo!(),
    };
    account_state_ex
}
