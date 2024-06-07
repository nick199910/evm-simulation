use std::collections::{BTreeMap, HashMap};
use std::{env, fmt, process};
use std::sync::Arc;
use ethers::addressbook::Address;
use ethers::prelude::{GethDebugBuiltInTracerConfig, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingOptions, Http, PreStateConfig, PreStateFrame, Provider, ProviderExt};
use ethers::prelude::GethDebugBuiltInTracerType::PreStateTracer;
use ethers::providers::Middleware;
use ethers::types::AccountState;
use ethers::utils::keccak256;
use primitive_types::H256;
use crate::core_module::utils::bytes::to_h256;
use crate::core_module::context::transaction_context::StateTracerType;
use crate::core_module::utils::bytes::pad_left;

#[derive(Debug, Clone)]
pub struct AccountStateEx {
    /// The account's nonce, which is incremented each time a transaction is sent from the account.
    pub nonce: u64,

    /// The account's balance, represented as a 32-byte array.
    pub balance: [u8; 32],

    /// The account's storage, represented as a hashmap where the keys and values are both 32-byte arrays.
    pub storage: Option<HashMap<[u8; 32], [u8; 32]>>,

    /// The hash of the account's code, represented as a 32-byte array.
    pub code_hash: Option<[u8; 32]>,
    pub code: Option<String>,
    pub state_tracer_type: StateTracerType,
}

// 实现 Display trait 用于格式化打印
impl fmt::Display for AccountStateEx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Nonce: {}", self.nonce)?;
        writeln!(f, "Balance: {:?}", self.balance)?;

        if let Some(storage) = &self.storage {
            writeln!(f, "Storage:")?;
            for (key, value) in storage {
                writeln!(f, "  {:?} -> {:?}", key, value)?;
            }
        } else {
            writeln!(f, "Storage: None")?;
        }

        if let Some(code_hash) = &self.code_hash {
            writeln!(f, "Code Hash: {:?}", code_hash)?;
        } else {
            writeln!(f, "Code Hash: None")?;
        }
        writeln!(f, "State Tracer Type: {:?}", self.state_tracer_type)?;
        Ok(())
    }
}


#[derive(Debug, Clone)]
pub struct ISDiff {
    /// is diff
    pub is_diff: bool,
    /// turn on diff and judge is_pre
    pub is_pre: Option<bool>
}

impl ISDiff {
    pub fn default() -> Self {
        Self{
            is_diff: false,
            is_pre: None,
        }
    }

    pub fn new(is_diff: bool, is_pre: Option<bool>) -> Self {
        Self {
            is_diff,
            is_pre
        }
    }
}

// 拿到交易执行之前的账户状态
pub async fn get_turn_off_diff_accounts_state(
    provider: Arc<Provider<Http>>,
    tx_hash: H256
) -> BTreeMap<Address, AccountStateEx> {
    get_accounts_state_tx(provider, tx_hash, ISDiff::default()).await
}
// 拿到交易之后返回的pre字段中对应的账户状态
pub async fn get_turn_on_diff_pre_accounts_state(
    provider: Arc<Provider<Http>>,
    tx_hash: H256
) -> BTreeMap<Address, AccountStateEx> {
    get_accounts_state_tx(provider, tx_hash, ISDiff::new(true, Some(true))).await
}
// 拿到交易执行之后的post字段中对应的账户状态
pub async fn get_turn_on_diff_post_accounts_state(
    provider: Arc<Provider<Http>>,
    tx_hash: H256
) -> BTreeMap<Address, AccountStateEx> {
    get_accounts_state_tx(provider, tx_hash, ISDiff::new(true, Some(false))).await
}

// 拿到交易执行之后读取和写入的账户状态
pub async fn get_tx_after_accounts_state(
    provider: Arc<Provider<Http>>,
    tx_hash: H256
) -> BTreeMap<Address, AccountStateEx> {

    let mut turn_off_diff_accounts_state = get_turn_off_diff_accounts_state(provider.clone(), tx_hash).await;
    let mut turn_on_diff_post_accounts_state = get_turn_on_diff_post_accounts_state(provider, tx_hash).await;

    for (key, value) in turn_on_diff_post_accounts_state {
        if let Some(existing_value) = turn_off_diff_accounts_state.get_mut(&key) {

            // 逐个字段替换
            existing_value.nonce = value.nonce;
            existing_value.balance = value.balance;

            if let Some(storage) = value.storage {
                if let Some(existing_storage) = existing_value.storage.as_mut() {
                    for (k, v) in storage {
                        if existing_storage.contains_key(&k) {
                            existing_storage.insert(k, v);
                        }
                    }
                }
            }
            if let Some(code_hash) = value.code_hash {
                existing_value.code_hash = Some(code_hash);
            }
            if let Some(code) = value.code {
                existing_value.code = Some(code);
            }
            existing_value.state_tracer_type = StateTracerType::TXAfterState;
        }
    }
    turn_off_diff_accounts_state
}

// 将完成的新的账户状态进行插入
pub fn insert_tx_account_state_ex(
    mut tx_account_state_ex: BTreeMap<Address, AccountStateEx>,
    new_tx_account_state_ex: &BTreeMap<Address, AccountState>,
    isdiff: ISDiff
) -> BTreeMap<Address, AccountStateEx> {
    new_tx_account_state_ex.iter().for_each(|(_addr, _account_state)| {
        let balance_u8 = if _account_state.balance.is_some() {
             let mut balance_u64 = _account_state.balance.clone().unwrap();
            let mut balance_u8 = vec![0u8; 32];
            balance_u64.to_big_endian(balance_u8.as_mut());
            balance_u8
        } else {
            vec![0u8; 32]
        };

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

        let state_tracer_type = if isdiff.is_diff == false {
            StateTracerType::TurnOffDiff
        } else {
            let state_tracer_type = if isdiff.is_pre.unwrap() == true {
                StateTracerType::TurnOnDiffPre
            } else {
                StateTracerType::TurnOnDiffPost
            };
            state_tracer_type
        };

        let mut account_state = AccountStateEx {
            nonce,
            balance: pad_left(&balance_u8),
            storage: Some(my_storage),
            code_hash,
            code,
            state_tracer_type,
        };
        tx_account_state_ex.insert(*_addr, account_state.clone());
    });
    tx_account_state_ex
}


pub async fn get_accounts_state_tx(
    provider: Arc<Provider<Http>>,
    tx_hash: H256,
    is_diff: ISDiff,
) -> BTreeMap<Address, AccountStateEx> {
    let tracer_config = GethDebugTracerConfig::BuiltInTracer(
        GethDebugBuiltInTracerConfig::PreStateTracer(PreStateConfig {
            diff_mode: Some(is_diff.is_diff),
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
    // println!("PreStatetracer difference：{:?}",tracer_info);
    let mut tx_account_state_ex: BTreeMap<Address, AccountStateEx> = BTreeMap::new();

    match tracer_info {
        ethers::types::GethTrace::Known(ref geth_tracer_frame) => match geth_tracer_frame {
            ethers::types::GethTraceFrame::PreStateTracer(pre_state_frame ) => match pre_state_frame {
                PreStateFrame::Default(default_mode ) => {
                    let true_off_pre_state = &default_mode.0;
                    tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, true_off_pre_state, is_diff);
                }
                PreStateFrame::Diff(diff_on) => {
                    if is_diff.is_pre.unwrap() == true {
                        let turn_on_diff_pre_state = &diff_on.pre;
                        println!("This is diff_on.pre:{:?}",turn_on_diff_pre_state);
                        tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, turn_on_diff_pre_state, is_diff);
                    } else {
                        let turn_on_diff_pre_state = &diff_on.post;
                        println!("This is diff_on.post:{:?}",turn_on_diff_pre_state);
                        tx_account_state_ex = insert_tx_account_state_ex(tx_account_state_ex, turn_on_diff_pre_state, is_diff);
                    }
                }
            },
            _ => todo!(),
        },
        _ => todo!(),
    };
    tx_account_state_ex
}


#[tokio::test]
pub async fn test_get_turn_off_diff_accounts_state() {
    let provider_http_url = String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b");

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");
    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let account_state = get_accounts_state_tx(Arc::from(provider), to_h256(attack_hash), ISDiff::default()).await;
    // println!("{:?}", account_state);
}

#[tokio::test]
pub async fn test_get_turn_on_diff_pre_accounts_state() {
    let provider_http_url = String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b");

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");
    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let account_state = get_accounts_state_tx(Arc::from(provider), to_h256(attack_hash), ISDiff::new(true, Some(true))).await;
    // println!("{:?}", account_state);
}

#[tokio::test]
pub async fn test_get_turn_on_diff_post_accounts_state() {
    let provider_http_url = String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b");

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");
    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let account_state = get_accounts_state_tx(Arc::from(provider), to_h256(attack_hash), ISDiff::new(true, Some(false))).await;
    // println!("{:?}", account_state);
}

#[tokio::test]
pub async fn test_get_tx_after_accounts_state() {
    let provider_http_url = String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b");

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");

    let attack_hash = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";
    let account_state = get_tx_after_accounts_state(Arc::from(provider), to_h256(attack_hash)).await;
    // println!("{:?}", account_state);
}


