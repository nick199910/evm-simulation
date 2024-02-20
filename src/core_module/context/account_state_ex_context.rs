use std::collections::{BTreeMap, HashMap};
use std::process;
use std::sync::Arc;
use ethers::addressbook::Address;
use ethers::prelude::{GethDebugBuiltInTracerConfig, GethDebugTracerConfig, GethDebugTracerType, GethDebugTracingOptions, Http, PreStateConfig, PreStateFrame, Provider};
use ethers::prelude::GethDebugBuiltInTracerType::PreStateTracer;
use ethers::providers::Middleware;
use ethers::utils::keccak256;
use primitive_types::H256;
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


pub async fn get_accounts_state_pre_tx(
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
