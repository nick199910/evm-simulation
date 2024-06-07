use crate::bytes::{_hex_string_to_bytes, pad_left, to_h160};
use crate::core_module::context::account_state_ex_context::{
    get_accounts_state_tx, AccountStateEx, ISDiff,
};
use crate::core_module::context::calldata_info::CallDataInfo;
use crate::core_module::context::evm_context::EvmContext;
use crate::core_module::context::transaction_context::get_transaction_content;
use crate::core_module::runner::{self, Runner};
use crate::core_module::state::{AccountState, EvmState};
use crate::core_module::utils::bytes::to_h256;
use crate::{Memory, Stack};
use alloy_primitives::utils::parse_ether;
use alloy_rlp::Bytes;
use dotenv::dotenv;
use ethers::prelude::Provider;
use ethers::providers::{Middleware, ProviderError, ProviderExt};
use ethers::types::Opcode::SHA3;
use ethers::types::{Address, TxHash};
use ethers::utils::keccak256;
use hex::FromHex;
use serde::de;
pub use serde::Deserialize;
pub use serde::Serialize;
use std::cell::RefCell;
use std::collections::{BTreeMap, HashMap};
use std::fmt::Debug;
use std::os::unix::process::parent_id;
use std::rc::Rc;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use std::{clone, env, fmt};

// 需要传入的数据为rpc, tx_hash, 要监控的函数字符串, 参数位置，(未解决的问题: 不变量如何传入)
// 参数变异策略
// 路径对比策略
// 需要有选择性的更改执行的上下文

#[tokio::test]
async fn test_tx_state() -> Result<(), ProviderError> {
    // 1. set provider
    dotenv().ok().expect(".env file not exit");
    let provider_http_url = env::var("mainnet").unwrap_or_else(|_| {
        String::from("https://lb.nodies.app/v1/181a5ebf4c954f8496ae7cbc1ac8d03b")
    });

    let provider = Provider::try_connect(provider_http_url.as_str())
        .await
        .expect("rpc connect error");

    let olympus_dao_tx = "0x3ed75df83d907412af874b7998d911fdf990704da87c2b1a8cf95ca5d21504cf";

    // 2. Obtain the pre_transaction_account_state, 需要把这个状态改为post的状态
    let accounts_state_pre_tx = get_accounts_state_tx(
        Arc::new(provider.clone()),
        to_h256(olympus_dao_tx),
        ISDiff::default(),
    )
    .await;

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

    println!("Caller is :{:?}", caller);
    println!("Address is :{:?}", address);
    println!("data is :{:?}", data);

    // 5. Create a new interpreter
    let mut interpreter = Runner::new_paper(
        caller,
        Some(origin),
        Some(address),
        Some(value),
        Some(data),
        Some(state),
        None,
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

    // update calldata
    let mut calldata_info = CallDataInfo::new();
    let origin_data =
        _hex_string_to_bytes("0x00000000000000000000000000000000000000000000000000001baeaf3816f8");
    calldata_info.origin = origin_data.clone();
    println!("{:?}", calldata_info.origin);
    interpreter.calldata_info = Some(calldata_info);

    // exec bytecode
    let bytecode = accounts_state_pre_tx
        .get(&Address::from_slice(&transaction_content.to.unwrap()))
        .unwrap()
        .code
        .as_ref()
        .unwrap();

    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");
        println!("bytecode is :{:?}", bytecode);
        let new_param = _hex_string_to_bytes(
            "0x00000000000000000000000000000000000000000000000000001baeaf3816f6",
        );
        // Interpret the bytecode
        let ret = interpreter.interpret_init(bytecode, new_param, true);
        if ret.is_ok() {
            println!("{:?}", interpreter.op_list.len());
            println!("successful!!!!");
        } else {
            println!("fail!!!!!!")
        }
    }

    Ok(())
}

#[test]
fn test_call_betweenDiffContract() {
    let addr1 = "5e17b14ADd6c386305A32928F985b29bbA34Eff5";
    let addr2 = "e2899bddFD890e320e643044c6b95B9B0b84157A";

    let bytecode1 = hex::decode("608060405234801561001057600080fd5b5060e38061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c8063a1c51915146037578063d46300fd146051575b600080fd5b603d606b565b60405160489190608a565b60405180910390f35b60576074565b60405160629190608a565b60405180910390f35b60006002905090565b60006001905090565b60848160a3565b82525050565b6000602082019050609d6000830184607d565b92915050565b600081905091905056fea2646970667358221220e563928ccb0a6664376561993a7a30a2cb5e8f130cec220db17bd0ee1d465f8464736f6c63430008030033").expect("Invalid bytecode");

    let codehash1 = keccak256(bytecode1.clone());
    let bytecode2 = hex::decode("608060405273ef9f1ace83dfbb8f559da621f4aea72c6eb10ebf6000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561006457600080fd5b506105be806100746000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063312d999c1461003b578063d4b839921461006b575b600080fd5b6100556004803603810190610050919061038f565b610089565b604051610062919061044c565b60405180910390f35b610073610318565b6040516100809190610431565b60405180910390f35b6000806040516024016040516020818303038152906040527fd46300fd000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060006040516024016040516020818303038152906040527fa1c51915000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16846040516101e0919061041a565b6000604051808303816000865af19150503d806000811461021d576040519150601f19603f3d011682016040523d82523d6000602084013e610222565b606091505b509150915060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168560405161026f919061041a565b6000604051808303816000865af19150503d80600081146102ac576040519150601f19603f3d011682016040523d82523d6000602084013e6102b1565b606091505b50915091506000838060200190518101906102cc9190610366565b90506000828060200190518101906102e49190610366565b9050898b82846102f4919061047d565b6102fe919061047d565b610308919061047d565b9850505050505050505092915050565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60008135905061034b81610571565b92915050565b60008151905061036081610571565b92915050565b60006020828403121561037857600080fd5b600061038684828501610351565b91505092915050565b600080604083850312156103a257600080fd5b60006103b08582860161033c565b92505060206103c18582860161033c565b9150509250929050565b6103d4816104d3565b82525050565b60006103e582610467565b6103ef8185610472565b93506103ff81856020860161050f565b80840191505092915050565b61041481610505565b82525050565b600061042682846103da565b915081905092915050565b600060208201905061044660008301846103cb565b92915050565b6000602082019050610461600083018461040b565b92915050565b600081519050919050565b600081905092915050565b600061048882610505565b915061049383610505565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff038211156104c8576104c7610542565b5b828201905092915050565b60006104de826104e5565b9050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b60005b8381101561052d578082015181840152602081019050610512565b8381111561053c576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b61057a81610505565b811461058557600080fd5b5056fea2646970667358221220f2e119cc89f32f22c3824fded0cfe597808ffa846a09d4d6020ae692fd1ac79464736f6c63430008030033").expect("Invalid bytecode");
    let codehash2 = keccak256(bytecode2.clone());

    let caller = _hex_string_to_bytes(addr1).try_into().unwrap();
    let origin = [
        0x5B, 0x38, 0xDa, 0x6a, 0x70, 0x1c, 0x56, 0x85, 0x45, 0xdC, 0xfc, 0xB0, 0x3F, 0xCB, 0x87,
        0x5f, 0x56, 0xbe, 0xdd, 0xC4,
    ];
    let address1 =
        <[u8; 20]>::from_hex("Ef9f1ACE83dfbB8f559Da621f4aEA72C6EB10eBf").expect("invaild address");
    let address2 =
        <[u8; 20]>::from_hex("0498B7c793D7432Cd9dB27fb02fc9cfdBAfA1Fd3").expect("invaild address");
    let inputdata = hex::decode("312d999c00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002").expect("Invalid bytecode");

    // 第一个合约，被调用合约，没有storage
    let mut account_state1 = AccountState::default();
    // account_state1.code_hash = codehash1;
    let mut account_state2 = AccountState {
        nonce: 0,
        balance: [0u8; 32],
        storage: HashMap::new(),
        code_hash: codehash2,
    };
    account_state2.storage.insert(
        <[u8; 32]>::from(to_h256(
            "0000000000000000000000000000000000000000000000000000000000000000",
        )),
        <[u8; 32]>::from(to_h256(
            "000000000000000000000000ef9f1ace83dfbb8f559da621f4aea72c6eb10ebf",
        )),
    );
    let mut state = EvmState::new(None);
    state.accounts.insert(
        <[u8; 20]>::from(Address::from(address1)),
        account_state1.clone(),
    );
    state.accounts.insert(
        <[u8; 20]>::from(Address::from(address2)),
        account_state2.clone(),
    );

    let mut interpreter = Runner::new(
        caller,
        Some(origin),
        Some(address1),
        None,
        None,
        Some(state.clone()),
        None,
    );
    // interpreter.modify_account_state(address1,account_state1);

    let ret = interpreter.interpret(bytecode1, true);
    // println!("After interpreter,account_state Is {:?}",interpreter.state.codes);
    // println!("EVMState is :{:?}",interpreter.state);

    if ret.is_ok() {
        println!("oplist_len is :{:?}", interpreter.op_list.len());
        println!("oplist is :{:?}", interpreter.op_list);
    } else {
        println!("ret is {:?}", ret);
    }

    interpreter.caller = origin;
    interpreter.origin = origin;
    interpreter.address = address2;
    interpreter.calldata = Memory::new(Option::from(vec![0u8]));

    let ret2 = interpreter.interpret(bytecode2, true);
    if ret2.is_ok() {
        println!("oplist_len is :{:?}", interpreter.op_list.len());
        println!("oplist is :{:?}", interpreter.op_list);
    } else {
        println!("ret is {:?}", ret2.unwrap());
    }
    interpreter.call(address2, [0; 32], inputdata, 0, false);
}

#[tokio::test]
async fn test_keccak256() {
    let mut bytecode1 = "60606040523415600e57600080fd5b603580601b6000396000f3006060604052600080fd00a165627a7a72305820395f38545a3c60d8fb3628f35d8ed9df7363257889a1311469f57957730033870029";
    let mut hash = keccak256(bytecode1);
    println!("codehash is :{:?}", hash);
}

#[test]
fn test_one() {
    // 部署者地址设置
    let _deployer = "5e17b14ADd6c386305A32928F985b29bbA34Eff5";
    let deployer: [u8; 20] = _hex_string_to_bytes(_deployer).try_into().unwrap();

    // 字节码
    let bytecode1 = hex::decode("608060405234801561001057600080fd5b5060e38061001f6000396000f3fe6080604052348015600f57600080fd5b506004361060325760003560e01c8063a1c51915146037578063d46300fd146051575b600080fd5b603d606b565b60405160489190608a565b60405180910390f35b60576074565b60405160629190608a565b60405180910390f35b60006002905090565b60006001905090565b60848160a3565b82525050565b6000602082019050609d6000830184607d565b92915050565b600081905091905056fea2646970667358221220e563928ccb0a6664376561993a7a30a2cb5e8f130cec220db17bd0ee1d465f8464736f6c63430008030033").expect("Invalid bytecode");
    let bytecode2 = hex::decode("608060405273ef9f1ace83dfbb8f559da621f4aea72c6eb10ebf6000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff16021790555034801561006457600080fd5b506105be806100746000396000f3fe608060405234801561001057600080fd5b50600436106100365760003560e01c8063312d999c1461003b578063d4b839921461006b575b600080fd5b6100556004803603810190610050919061038f565b610089565b604051610062919061044c565b60405180910390f35b610073610318565b6040516100809190610431565b60405180910390f35b6000806040516024016040516020818303038152906040527fd46300fd000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060006040516024016040516020818303038152906040527fa1c51915000000000000000000000000000000000000000000000000000000007bffffffffffffffffffffffffffffffffffffffffffffffffffffffff19166020820180517bffffffffffffffffffffffffffffffffffffffffffffffffffffffff8381831617835250505050905060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16846040516101e0919061041a565b6000604051808303816000865af19150503d806000811461021d576040519150601f19603f3d011682016040523d82523d6000602084013e610222565b606091505b509150915060008060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff168560405161026f919061041a565b6000604051808303816000865af19150503d80600081146102ac576040519150601f19603f3d011682016040523d82523d6000602084013e6102b1565b606091505b50915091506000838060200190518101906102cc9190610366565b90506000828060200190518101906102e49190610366565b9050898b82846102f4919061047d565b6102fe919061047d565b610308919061047d565b9850505050505050505092915050565b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1681565b60008135905061034b81610571565b92915050565b60008151905061036081610571565b92915050565b60006020828403121561037857600080fd5b600061038684828501610351565b91505092915050565b600080604083850312156103a257600080fd5b60006103b08582860161033c565b92505060206103c18582860161033c565b9150509250929050565b6103d4816104d3565b82525050565b60006103e582610467565b6103ef8185610472565b93506103ff81856020860161050f565b80840191505092915050565b61041481610505565b82525050565b600061042682846103da565b915081905092915050565b600060208201905061044660008301846103cb565b92915050565b6000602082019050610461600083018461040b565b92915050565b600081519050919050565b600081905092915050565b600061048882610505565b915061049383610505565b9250827fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff038211156104c8576104c7610542565b5b828201905092915050565b60006104de826104e5565b9050919050565b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000819050919050565b60005b8381101561052d578082015181840152602081019050610512565b8381111561053c576000848401525b50505050565b7f4e487b7100000000000000000000000000000000000000000000000000000000600052601160045260246000fd5b61057a81610505565b811461058557600080fd5b5056fea2646970667358221220f2e119cc89f32f22c3824fded0cfe597808ffa846a09d4d6020ae692fd1ac79464736f6c63430008030033").expect("Invalid bytecode");

    // 合约地址 => 这里可以随便设置
    let contract_address1 =
        <[u8; 20]>::from_hex("Ef9f1ACE83dfbB8f559Da621f4aEA72C6EB10eBf").expect("invaild address");
    let contract_address2 =
        <[u8; 20]>::from_hex("0498B7c793D7432Cd9dB27fb02fc9cfdBAfA1Fd3").expect("invaild address");

    // 做第一次部署
    let mut interpreter = Runner::new(
        deployer,
        Some(deployer),
        Some(contract_address1),
        None,
        None,
        Some(EvmState::new(None)), // 给定空的状态，做初始化
        None,
    );
    let contract1_deploy = interpreter.interpret(bytecode1, true);
    assert_eq!(contract1_deploy.unwrap(), ());

    // 第二次部署前，修改to地址
    interpreter.address = contract_address2;

    // 第二次部署
    let contract2_deploy = interpreter.interpret(bytecode2, true);
    assert_eq!(contract2_deploy.unwrap(), ());

    // call之前，设置storage值
    let set_result = interpreter.set_storage(
        &contract_address2,
        [0; 32],
        <[u8; 32]>::from(to_h256(
            "000000000000000000000000ef9f1ace83dfbb8f559da621f4aea72c6eb10ebf",
        )),
    );
    assert_eq!((), set_result.unwrap());

    // call
    let inputdata = hex::decode("312d999c00000000000000000000000000000000000000000000000000000000000000010000000000000000000000000000000000000000000000000000000000000002").expect("Invalid bytecode");
    let call_ret = interpreter.call(contract_address2, [0; 32], inputdata, 0, false);
    assert_eq!(call_ret.unwrap(), ());
}

// #[test]
// fn test_reentrancy() {
//     // 合约部署者
//     let deployer = TryInto::<[u8; 20]>::try_into(Address::random()).unwrap();
//     let valut_balance: [u8; 32] = parse_ether("10")
//         .unwrap()
//         .to_be_bytes_vec()
//         .try_into()
//         .expect("transfer error");
//     println!("{:?}", valut_balance);
//     // 初始化evm
//     let mut runner = Runner::new(
//         deployer,
//         Some(deployer),
//         None,
//         Some(valut_balance),
//         None,
//         Some(EvmState::new(None)),
//         None,
//     );

//     // 字节码
//     let valut_bytecode = hex::decode("6080604052610327806100136000396000f3fe6080604052600436106100345760003560e01c80633ccfd60b1461003957806370a0823114610050578063d0e30db01461008d575b600080fd5b34801561004557600080fd5b5061004e610097565b005b34801561005c57600080fd5b506100776004803603810190610072919061024a565b61018a565b6040516100849190610290565b60405180910390f35b6100956101a2565b005b60008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000205490503373ffffffffffffffffffffffffffffffffffffffff16816040516100fe906102dc565b60006040518083038185875af1925050503d806000811461013b576040519150601f19603f3d011682016040523d82523d6000602084013e610140565b606091505b50505060008060003373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1681526020019081526020016000208190555050565b60006020528060005260406000206000915090505481565b346000803373ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16815260200190815260200160002081905550565b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b6000610217826101ec565b9050919050565b6102278161020c565b811461023257600080fd5b50565b6000813590506102448161021e565b92915050565b6000602082840312156102605761025f6101e7565b5b600061026e84828501610235565b91505092915050565b6000819050919050565b61028a81610277565b82525050565b60006020820190506102a56000830184610281565b92915050565b600081905092915050565b50565b60006102c66000836102ab565b91506102d1826102b6565b600082019050919050565b60006102e7826102b9565b915081905091905056fea26469706673582212200f547a969111f372811a11deef02a4096a2e0ba2c2e8795ab4840ee920f6efec64736f6c63430008130033").unwrap();
//     // 部署合约
//     let valut_address = runner.deploy_contract(valut_bytecode, vec![]).unwrap();
//     println!("valut address {:?}", valut_address);
//     println!("valut {:?}", runner.state.accounts.get(&valut_address));

//     let hacker_bytecode = hex::decode("6080604052604051610318380380610318833981810160405281019061002591906100ce565b806000806101000a81548173ffffffffffffffffffffffffffffffffffffffff021916908373ffffffffffffffffffffffffffffffffffffffff160217905550506100fb565b600080fd5b600073ffffffffffffffffffffffffffffffffffffffff82169050919050565b600061009b82610070565b9050919050565b6100ab81610090565b81146100b657600080fd5b50565b6000815190506100c8816100a2565b92915050565b6000602082840312156100e4576100e361006b565b5b60006100f2848285016100b9565b91505092915050565b61020e8061010a6000396000f3fe6080604052600436106100225760003560e01c80634de260a2146100b657610023565b5b6729a2241af62c00004710156100b45760008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633ccfd60b6040518163ffffffff1660e01b8152600401600060405180830381600087803b15801561009b57600080fd5b505af11580156100af573d6000803e3d6000fd5b505050505b005b3480156100c257600080fd5b506100cb6100cd565b005b60008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff1663d0e30db0670de0b6b3a76400006040518263ffffffff1660e01b81526004016000604051808303818588803b15801561013d57600080fd5b505af1158015610151573d6000803e3d6000fd5b505050505060008054906101000a900473ffffffffffffffffffffffffffffffffffffffff1673ffffffffffffffffffffffffffffffffffffffff16633ccfd60b6040518163ffffffff1660e01b8152600401600060405180830381600087803b1580156101be57600080fd5b505af11580156101d2573d6000803e3d6000fd5b5050505056fea2646970667358221220bf38f216eba291adff1b00e83cc5ff4bc5071b2c92f8054082bdbb84fd836f6e64736f6c63430008130033").unwrap();
//     // 参数
//     let mut params = vec![];
//     params.insert(0, pad_left(valut_address.as_slice()));
//     let hacker_balance: [u8; 32] = parse_ether("1")
//         .unwrap()
//         .to_be_bytes_vec()
//         .try_into()
//         .expect("transfer error");
//     runner.callvalue = hacker_balance;
//     // 部署hacker
//     let hacker_address = runner.deploy_contract(hacker_bytecode, params).unwrap();
//     println!("hacker address {:?}", hacker_address);
//     println!("hacker {:?}", runner.state.accounts.get(&hacker_address));

//     // 执行交易
//     runner.call(
//         hacker_address,
//         [0; 32],
//         hex::decode("4de260a2").unwrap(),
//         0,
//         false,
//     );
//     println!("hacker {:?}", runner.state.accounts.get(&hacker_address));
//     let mut call_data = hex::decode("70a08231").unwrap();
//     let param = pad_left(hacker_address.as_slice()).to_vec();
//     call_data.extend(param);
//     runner.call(valut_address, [0; 32], call_data, 0, false);
// }
