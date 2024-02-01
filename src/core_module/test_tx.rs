use std::collections::{BTreeMap, HashMap};
use std::mem::transmute;
use std::process;
use std::str::FromStr;
use std::sync::Arc;
use ethers::addressbook::Address;
use ethers::prelude::{Bytes, GethDebugBuiltInTracerConfig, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingOptions, Http, PreStateConfig, PreStateFrame, Provider, U64};
use ethers::prelude::GethDebugBuiltInTracerType::PreStateTracer;
use ethers::providers::{Middleware, ProviderError, ProviderExt};
use ethers::types::{PreStateMode, Res, TxHash};
use ethers::utils::keccak256;
use primitive_types::{H256, U256};
pub use serde::Deserialize;
use super::memory::Memory;
pub use serde::Serialize;
use crate::core_module::runner::Runner;
use crate::core_module::state::EvmState;
use crate::core_module::test_account::AccountStateEx;
use crate::core_module::utils::bytes::{pad_left, to_h256};

#[derive(Debug, Clone)]
pub struct Transaction {
    /// The transaction's hash
    pub hash: H256,

    /// The transaction's nonce
    pub nonce: U256,

    /// Block hash. None when pending.
    pub block_hash: Option<H256>,

    pub block_number: Option<U64>,

    pub transaction_index: Option<U64>,

    pub from: Address,

    pub to: Option<Address>,

    /// Transferred value
    pub value: U256,

    pub gas_price: Option<U256>,

    /// Gas amount
    pub gas: U256,

    /// Input data
    pub calldata: Memory,

    pub transaction_type: Option<U64>,


    pub max_priority_fee_per_gas: Option<U256>,
    
    pub max_fee_per_gas: Option<U256>,

    pub chain_id: Option<U256>,

}


async fn get_transaction_content(provider: Provider<Http>, tx_hash: TxHash) -> Result<Transaction, ProviderError> {

    let transaction = provider.get_transaction(tx_hash).await.expect("get transaction hash error");

    let calldata = transaction.clone().unwrap().input.to_vec();

    println!("{:?}", calldata);

    Ok(Transaction{
        hash: transaction.clone().unwrap().hash,
        nonce: transaction.clone().unwrap().nonce,
        block_hash: transaction.clone().unwrap().block_hash,
        block_number: transaction.clone().unwrap().block_number,
        transaction_index: transaction.clone().unwrap().transaction_index,
        from: transaction.clone().unwrap().from,
        to: transaction.clone().unwrap().to,
        value: transaction.clone().unwrap().value,
        gas_price: transaction.clone().unwrap().gas_price,
        gas: transaction.clone().unwrap().gas,
        calldata: Memory::new(Some(calldata)),
        transaction_type: transaction.clone().unwrap().transaction_type,
        max_priority_fee_per_gas: transaction.clone().unwrap().max_priority_fee_per_gas,
        max_fee_per_gas: transaction.clone().unwrap().max_fee_per_gas,
        chain_id: transaction.clone().unwrap().chain_id,
    })
}


#[derive(Debug, Clone)]
pub(crate) enum StateTracerType {
    None,
    TurnOffDiff,
    TurnOnDiffPre,
    TurnOnDiffPost
}


fn get_test_account_msg() -> AccountStateEx {
    let code = "0x608060405234801561000f575f80fd5b506004361061003f575f3560e01c80633fb5c1cb14610043578063893d20e81461005f578063a6f9dae114610069575b5f80fd5b61005d6004803603810190610058919061011e565b610085565b005b61006761009a565b005b610083600480360381019061007e91906101a3565b6100a5565b005b600c8110156100975760056001819055505b50565b6100a3336100a5565b565b805f806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555050565b5f80fd5b5f819050919050565b6100fd816100eb565b8114610107575f80fd5b50565b5f81359050610118816100f4565b92915050565b5f60208284031215610133576101326100e7565b5b5f6101408482850161010a565b91505092915050565b5f73ffffffffffffffffffffffffffffffffffffffff82169050919050565b5f61017282610149565b9050919050565b61018281610168565b811461018c575f80fd5b50565b5f8135905061019d81610179565b92915050565b5f602082840312156101b8576101b76100e7565b5b5f6101c58482850161018f565b9150509291505056fea26469706673582212205583d88608fb17547ba77bd69a991b8647738b63e2a17f2d3a0c5dfd1fa118a364736f6c63430008160033";
    AccountStateEx{
        nonce: 1,
        balance:  pad_left(&[0x00]),
        storage: Default::default(),
        code_hash: Some(keccak256(code)),
        code: Some(code.to_string()),
        state_tracer_type: StateTracerType::None
    }
}





#[tokio::test]
async fn test_get_transaction_content() -> Result<(), ProviderError>{
    let sepolia_rpc = "https://rough-frosty-field.ethereum-sepolia.quiknode.pro/40fa9bf59d6007a200145efb93862af9a528e8ae/";

    let set_five_hash = "0xceb77591c14a3a8458741a0a1e205e56d319f970c6ef497a41917df464401561";
    let set_six_hash = "0x55e59ddfb18232d0e23e5b9675900ea3e614490c5e801fa75a3cf572ebf70ca2";

    let provider = Provider::try_connect(sepolia_rpc).await.expect("rpc connect error");

    let transaction_content = get_transaction_content(provider, TxHash::from_str(set_six_hash).unwrap()).await.expect("get transaction hash error");

    let account_msg = get_test_account_msg();


    Ok(())

    // let ret = provider.get_transaction(TxHash::from_str(set_five_hash).unwrap()).await?;
}

#[tokio::test]
async fn test_no_state_run() -> Result<(), ProviderError> {

    let sepolia_rpc = "https://rough-frosty-field.ethereum-sepolia.quiknode.pro/40fa9bf59d6007a200145efb93862af9a528e8ae/";

    let set_five_hash = "0xceb77591c14a3a8458741a0a1e205e56d319f970c6ef497a41917df464401561";
    let set_six_hash = "0x55e59ddfb18232d0e23e5b9675900ea3e614490c5e801fa75a3cf572ebf70ca2";

    let provider = Provider::try_connect(sepolia_rpc).await.expect("rpc connect error");

    let transaction_content = get_transaction_content(provider, TxHash::from_str(set_six_hash).unwrap()).await.expect("get transaction hash error");
    let test_account_msg = get_test_account_msg();


    let mut caller = transaction_content.clone().from.0;
    let mut origin: Option<[u8; 20]> = None;
    let mut address: Option<[u8; 20]> = Some(transaction_content.to.unwrap().0);
    let mut value: Option<[u8; 32]> = None;
    let mut data: Option<Vec<u8>> = Some(transaction_content.calldata.heap);
    let mut bytecode: String = test_account_msg.code.unwrap();



    let state: EvmState;

    state = EvmState::new(None);


    // Create a new interpreter
    let mut interpreter =
        Runner::new(caller, origin, address, value, data, Some(state));

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
    let provider = Provider::try_connect(provider_http_url).await.expect("rpc connect error");

    let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e"; // 可行
    let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf"; // 可行
    let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5"; // 可行

    // 拿到交易前置账户状态
    let accounts_state_pre_tx = get_accounts_state_pre_tx(Arc::new(provider.clone()), to_h256(uniswap_v2_attack), false).await;

    // 对交易前置账户进行预处理

    // 拿到交易的上下文
    let transaction_content = get_transaction_content(provider, TxHash::from_str(uniswap_v2_attack).unwrap()).await.expect("get transaction hash error");

    // 拿到虚拟机
    let state: EvmState;
    state = EvmState::new(None);


    // 为虚拟机交易上下文
    let caller = transaction_content.from.0;
    let origin= transaction_content.from.0;
    let address = transaction_content.to.unwrap().0;

    let value_u64 = transaction_content.value.0;
    let mut value = [0u8; 32];

    for i in 0..4 {
        let bytes = value_u64[i].to_le_bytes(); // or to_be_bytes() for big-endian
        value[i * 8..(i + 1) * 8].copy_from_slice(&bytes);
    }

    let data = transaction_content.calldata.heap;

    // Create a new interpreter
    let mut interpreter =
        Runner::new(caller, Some(origin), Some(address), Some(value) , Some(data), Some(state));


    // 为虚拟机设置状态上下文
    // 枚举所有的状态
    accounts_state_pre_tx.iter().for_each(|(_addr, _account_state_ex)| {
        interpreter.modify_account_state(_addr.0, _account_state_ex.clone());
    });

    let bytecode = accounts_state_pre_tx.get(&transaction_content.to.unwrap()).unwrap().code.as_ref().unwrap();



    // // Check if bytecode is an hex value of a file path
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
async fn test_state_run() -> Result<(), ProviderError> {
    let provider_http_url = "https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b";
    let provider = Provider::try_connect(provider_http_url).await.expect("rpc connect error");


    let euler_attack = "0xc310a0affe2169d1f6feec1c63dbc7f7c62a887fa48795d327d4d2da2d6b111d";
    let uniswap_v2_attack = "0x45d108052e01c20f37fd05db462b9cef6629a70849bcd71b36291786ee6ee3e9";
    let usdc_transfer_tx = "0x890249a15f17950a60711c0396ccd147068365ea852f0837c08f55f9dd7c320e";
    let OlympusDAO_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let Templedao_tx = "0x8c3f442fc6d640a6ff3ea0b12be64f1d4609ea94edd2966f42c01cd9bdcf04b5";

    get_accounts_state_pre_tx(Arc::new(provider), to_h256(uniswap_v2_attack), false).await;

    Ok(())
}

async fn get_accounts_state_pre_tx(
    provider: Arc<Provider<Http>>,
    tx_hash: H256,
    is_diff: bool
) -> BTreeMap<Address, AccountStateEx>{

    let tracer_config = GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig { diff_mode: Some(is_diff) }),
    );

    let mut options = GethDebugTracingOptions::default();
    options.tracer = Some(GethDebugTracerType::BuiltInTracer(PreStateTracer));
    options.tracer_config = Some(tracer_config);


    let tracer_info = provider.debug_trace_transaction(tx_hash, options).await.unwrap_or_else(|err| {
        eprintln!("transaction reverted with err: {}", err);
        process::exit(1);
    });

    let mut account_state_ex: BTreeMap<Address, AccountStateEx> = BTreeMap::new();

    match tracer_info {
        ethers::types::GethTrace::Known(ref a) => match a {
            ethers::types::GethTraceFrame::PreStateTracer(b) => match b {
                PreStateFrame::Default(df) => {

                    let pre_state = &df.0;

                    pre_state.iter().for_each(
                        |(_addr, _account_state)| {
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
                            let mut my_storage:HashMap<[u8; 32], [u8; 32]> = HashMap::new();
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
                            account_state_ex.insert(
                                *_addr, account_state.clone()
                            );


                        }
                    );

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



