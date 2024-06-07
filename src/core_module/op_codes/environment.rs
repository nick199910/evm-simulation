use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::bytes::{bytes32_to_address, pad_left};
use crate::core_module::utils::environment::get_balance;
use crate::core_module::utils::errors::ExecutionError;
use std::f64::consts::E;
use std::time::{SystemTime, UNIX_EPOCH};

// Primitive types
use crate::core_module::context::account_state_ex_context;
use alloy_rlp::Encodable;
use ethers::types::U256;
use ethers::utils::keccak256;

pub fn address(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = pad_left(&runner.address);

    let result = runner.stack.push(address);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn balance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address: [u8; 32] = runner.stack.pop()?;
    let address: [u8; 20] = address[12..].try_into().unwrap();

    let balance = get_balance(address, runner)?;

    let result = runner.stack.push(pad_left(&balance));

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn origin(runner: &mut Runner) -> Result<(), ExecutionError> {
    let origin = pad_left(&runner.origin);

    let result = runner.stack.push(origin);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn caller(runner: &mut Runner) -> Result<(), ExecutionError> {
    let caller = pad_left(&runner.caller);

    let result = runner.stack.push(caller);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn callvalue(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.push(runner.callvalue);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldataload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let address = U256::from_big_endian(&address).as_usize();

    let calldata = unsafe { runner.calldata.read(address, 32)? };
    let calldata: [u8; 32] = calldata.as_slice().try_into().unwrap();
    // let result = runner.stack.push(calldata);

    /// todo! add calldata
    let origin_data_exist = runner.calldata_info.clone();
    let result = if origin_data_exist.is_some() {
        let origin_data = origin_data_exist.unwrap().origin;
        let mut new_calldata: Vec<u8> = Vec::new();
        let result = if Vec::from(calldata) == origin_data {
            new_calldata = runner.calldata_info.clone().unwrap().new;
            println!("{:?}", new_calldata);
            println!("发生替换!!!! {} ", runner.op_count);
            runner
                .stack
                .push(new_calldata.as_slice().try_into().unwrap())
        } else {
            runner.stack.push(calldata)
        };
        result
    } else {
        runner.stack.push(calldata)
    };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let size = runner.calldata.msize().to_be_bytes();

    // Convert the usize to bytes in little-endian order
    let calldatasize = pad_left(&size);

    let result = runner.stack.push(calldatasize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn calldatacopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let dest_offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let _offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let _size = U256::from_big_endian(&runner.stack.pop()?).as_usize();

    let calldata = unsafe { runner.calldata.read(_offset, _size)? };

    let result = unsafe { runner.memory.write(dest_offset, calldata) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn codesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    // let code = runner.state.get_code_at(runner.address);
    let code = &runner.bytecode;

    // let codesize = if code.is_none() {
    //     [0u8; 32]
    // } else {
    //     pad_left(&code.unwrap().len().to_be_bytes())
    // };
    let codesize = if code.length() == 0 {
        [0u8; 32]
    } else {
        pad_left(&code.len().to_be_bytes())
    };
    let result = runner.stack.push(codesize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn codecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    // 在memory中的offset
    let dest_offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    // 在bytecode的offset
    let offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    // code size
    let size = U256::from_big_endian(&runner.stack.pop()?).as_usize();

    // 用bytecode
    let code = &runner.bytecode;
    let mut copy_code = if code.len() == 0 {
        vec![]
    } else {
        // 确定要copy的代码
        code[offset..offset + size].to_vec()
    };
    let mut mem_size = 0;
    if dest_offset % 32 != 0 {
        mem_size += 1;
    }
    if (dest_offset + size) % 32 != 0 {
        mem_size += 1;
    }
    mem_size += size / 32;
    copy_code.resize(mem_size * 32, 0);
    // let data = copy_code.resize(, value)
    // Copy the code to memory
    unsafe { runner.memory.write(dest_offset, copy_code) }?;

    // Increment PC
    runner.increment_pc(1)
}

pub fn gasprice(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gasprice = match &runner.evm_context {
        None => pad_left(&[0xff]),
        Some(evm_context) => {
            if let Some(gas_price) = evm_context.gas_price {
                gas_price
            } else {
                pad_left(&[0xff])
            }
        }
    };

    let result = runner.stack.push(gasprice);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn extcodesize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;

    let code = runner.state.get_code_at(bytes32_to_address(&address));

    let codesize = if code.is_none() {
        [0u8; 32]
    } else {
        pad_left(&code.unwrap().len().to_be_bytes())
    };

    let result = runner.stack.push(codesize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn extcodecopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let dest_offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let size = U256::from_big_endian(&runner.stack.pop()?).as_usize();

    let code = runner.state.get_code_at(bytes32_to_address(&address));

    // Slice the code to the correct size
    let code = if code.is_none() {
        vec![]
    } else {
        // complete the code with 0s
        let code = code.unwrap();
        let mut code_vec = code.to_vec();
        code_vec.resize(32, 0);
        let code = code_vec.as_slice();
        code[offset..offset + size].to_vec()
    };

    // Copy the code to memory
    unsafe { runner.memory.write(dest_offset, code) }?;

    // Increment PC
    runner.increment_pc(1)
}

pub fn returndatasize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let size = runner.returndata.msize().to_be_bytes();

    // Convert the usize to bytes in little-endian order
    let returndatasize = pad_left(&size);

    let result = runner.stack.push(returndatasize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn returndatacopy(runner: &mut Runner) -> Result<(), ExecutionError> {
    let dest_offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let _offset = U256::from_big_endian(&runner.stack.pop()?).as_usize();
    let _size = U256::from_big_endian(&runner.stack.pop()?).as_usize();

    let returndata = unsafe { runner.returndata.read(_offset, _size)? };

    let result = unsafe { runner.memory.write(dest_offset, returndata) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn extcodehash(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;

    Ok(
        if let Some(code) = runner.state.get_code_at(bytes32_to_address(&address)) {
            let codehash = keccak256(code);
            let result = runner.stack.push(codehash);
            if result.is_err() {
                return Err(result.unwrap_err());
            }

            // Increment PC
            runner.increment_pc(1);
        },
    )
}

pub fn blockhash(runner: &mut Runner) -> Result<(), ExecutionError> {
    let block: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut bytes = [0; 32];
    block.to_big_endian(&mut bytes);

    let blockhash = keccak256(bytes);

    let result = runner.stack.push(blockhash);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn coinbase(runner: &mut Runner) -> Result<(), ExecutionError> {
    // let coinbase = pad_left(&[0xc0u8; 20]);

    let coinbase = match &runner.evm_context {
        None => pad_left(&[0xc0u8; 20]),
        Some(evm_context) => {
            if let Some(coinbase) = evm_context.coinbase {
                pad_left(&coinbase)
            } else {
                // Provide a default value if evm_context.coinbase is None
                pad_left(&[0xc0u8; 20])
            }
        }
    };

    let result = runner.stack.push(coinbase);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn timestamp(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Convert the timestamp to seconds
    let timestamp_secs = match &runner.evm_context {
        None => pad_left(&[0x00; 20]),
        Some(evm_context) => {
            if let Some(timestamp_secs) = evm_context.timestamp {
                timestamp_secs
            } else {
                pad_left(&[0x00; 20])
            }
        }
    };

    let result = runner.stack.push(timestamp_secs);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}
pub fn number(runner: &mut Runner) -> Result<(), ExecutionError> {
    let number = match &runner.evm_context {
        None => pad_left(&[0xff; 4]),
        Some(evm_context) => {
            if let Some(number) = evm_context.block_number {
                number
            } else {
                pad_left(&[0xff; 4])
            }
        }
    };

    let result = runner.stack.push(number);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

// 这个地方有疑问
pub fn difficulty(runner: &mut Runner) -> Result<(), ExecutionError> {
    let difficulty = pad_left(&[0x45; 8]);

    let result = runner.stack.push(difficulty);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn gaslimit(runner: &mut Runner) -> Result<(), ExecutionError> {
    let gaslimit = match &runner.evm_context {
        None => pad_left(&[0x01, 0xC9, 0xC3, 0x80]),
        Some(evm_context) => {
            if let Some(gaslimit) = evm_context.gas_limit {
                gaslimit
            } else {
                pad_left(&[0x01, 0xC9, 0xC3, 0x80])
            }
        }
    };

    let result = runner.stack.push(gaslimit);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn chainid(runner: &mut Runner) -> Result<(), ExecutionError> {
    let chainid = pad_left(&[0x01]);

    let result = runner.stack.push(chainid);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn selfbalance(runner: &mut Runner) -> Result<(), ExecutionError> {
    let balance = get_balance(runner.address, runner)?;

    let result = runner.stack.push(balance);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn basefee(runner: &mut Runner) -> Result<(), ExecutionError> {
    let basefee = match &runner.evm_context {
        None => pad_left(&[0x0a]),
        Some(evm_context) => {
            if let Some(basefee) = evm_context.basefee {
                basefee
            } else {
                pad_left(&[0x0a])
            }
        }
    };

    let result = runner.stack.push(basefee);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, _pad_right, pad_left};

    #[test]
    fn test_address() {
        let mut runner = Runner::_default();
        address(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&runner.address));
    }

    #[test]
    fn test_balance() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&runner.caller));
        balance(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(
            result,
            pad_left(&[0x36, 0x35, 0xC9, 0xAD, 0xC5, 0xDE, 0xA0, 0x00, 0x00])
        );

        // transfer 1 wei to the contract
        let _ = runner
            .state
            .transfer(runner.caller, runner.address, pad_left(&[0x01]));

        let _ = runner.stack.push(pad_left(&runner.caller));
        balance(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(
            result,
            pad_left(&[0x36, 0x35, 0xC9, 0xAD, 0xC5, 0xDE, 0x9F, 0xFF, 0xFF])
        );
    }

    #[test]
    fn test_origin() {
        let mut runner = Runner::_default();
        origin(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&runner.origin));
    }

    #[test]
    fn test_caller() {
        let mut runner = Runner::_default();
        caller(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&runner.caller));
    }

    #[test]
    fn test_callvalue() {
        let mut runner = Runner::_default();
        callvalue(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x00]));
    }

    #[test]
    fn test_calldataload() {
        let mut runner = Runner::_default();
        runner.calldata.heap = vec![0xff, 0xff, 0xff, 0xff];

        let _ = runner.stack.push(pad_left(&[0x00]));
        calldataload(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, _pad_right(&[0xff, 0xff, 0xff, 0xff]));

        let _ = runner.stack.push(pad_left(&[0x02]));
        calldataload(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, _pad_right(&[0xff, 0xff]));
    }

    #[test]
    fn test_calldatasize() {
        let mut runner = Runner::_default();
        runner.calldata.heap = vec![0xff, 0xff, 0xff, 0xff];

        calldatasize(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x04]));
    }

    #[test]
    fn test_calldatacopy() {
        let mut runner = Runner::_default();
        runner.calldata.heap = [0xff; 32].to_vec();

        let _ = runner.stack.push(pad_left(&[0x20]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32].to_vec());

        let _ = runner.stack.push(pad_left(&[0x10]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32].to_vec());

        runner.memory.heap = vec![0x00; 32];
        let _ = runner.stack.push(pad_left(&[0x10]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        calldatacopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, _pad_right(&[0xff; 16]).to_vec());
    }

    #[test]
    fn test_codesize() {
        let mut runner = Runner::_default();

        // Interpret some code to make set the runner code to something
        runner
            .interpret(_hex_string_to_bytes("60ff6000526001601ff3"), true)
            .unwrap();

        codesize(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xa]));
    }

    #[test]
    fn test_codecopy() {
        let mut runner = Runner::_default();

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            _hex_string_to_bytes(
                "7dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000",
            ),
            true,
        );
        assert!(interpret_result.is_ok());

        let _ = runner.stack.push(pad_left(&[0x20]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        codecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(
            result,
            _hex_string_to_bytes(
                "7dffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff60"
            )
        );

        // reset memory
        runner.memory.heap = vec![];

        let _ = runner.stack.push(pad_left(&[0x05]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        codecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, _pad_right(&_hex_string_to_bytes("7dffffffff")));
    }

    #[test]
    fn test_gasprice() {
        let mut runner = Runner::_default();
        gasprice(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xff]));
    }

    #[test]
    fn test_extcodesize() {
        let mut runner = Runner::_default();

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("7f76ffffffffffffffffffffffffffffffffffffffffffffff60005260176009f3600052602060006000f0"),
            true
        );
        assert!(interpret_result.is_ok());

        extcodesize(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x17]));
    }

    #[test]
    fn test_extcodecopy() {
        let mut runner = Runner::_default();

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("7f76ffffffffffffffffffffffffffffffffffffffffffffff60005260176009f3600052602060006000f0"),
            true
        );
        assert!(interpret_result.is_ok());

        // reset memory
        runner.memory.heap = vec![];

        let _ = runner.stack.push(pad_left(&[0x17]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.dup(4);
        extcodecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(
            result,
            _pad_right(&_hex_string_to_bytes(
                "ffffffffffffffffffffffffffffffffffffffffffffff"
            ))
        );

        // reset memory
        runner.memory.heap = vec![];

        let _ = runner.stack.push(pad_left(&[0xa]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        let _ = runner.stack.push(pad_left(&[0x20]));
        let _ = runner.stack.dup(4);
        extcodecopy(&mut runner).unwrap();

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0u8; 32]);
        let result = unsafe { runner.memory.read(0x20, 0x20).unwrap() };
        assert_eq!(
            result,
            _pad_right(&_hex_string_to_bytes("ffffffffffffffffffff"))
        );
    }

    #[test]
    fn test_returndatasize() {
        let mut runner = Runner::_default();

        // Create a contract that return 0x20 sized data and call it
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("7f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000527fff6000527fff60005260206000f30000000000000000000000000000000000006020527f000000000060205260296000f300000000000000000000000000000000000000604052604d60006000f060006000600060008463fffffffffa3d"),
            true
        );

        assert!(interpret_result.is_ok());

        let result = runner.stack.pop().unwrap();
        println!("{:?}", result);
        assert_eq!(result, pad_left(&[0x20]));
    }

    #[test]
    fn test_returndatacopy() {
        let mut runner = Runner::_default();

        // Create a contract that return 0x20 sized data and call it
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("7f7f7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff6000527fff6000527fff60005260206000f30000000000000000000000000000000000006020527f000000000060205260296000f300000000000000000000000000000000000000604052604d60006000f060006000600060008463fffffffffa50506000600052600060205260006040526020600060003e6001601f60203e"),
            true
        );
        assert!(interpret_result.is_ok());

        let result = unsafe { runner.memory.read(0x00, 0x20).unwrap() };
        assert_eq!(result, [0xff; 32]);
        let result = unsafe { runner.memory.read(0x20, 0x20).unwrap() };
        assert_eq!(result, _pad_right(&[0xff]));
        let result = unsafe { runner.memory.read(0x40, 0x20).unwrap() };
        assert_eq!(result, [0x00; 32]);
    }

    #[test]
    fn test_extcodehash() {
        let mut runner = Runner::_default();

        // Create a contract with a bytecode length of 23
        let interpret_result = runner.interpret(
            _hex_string_to_bytes("6c63ffffffff60005260046000f3600052600d60006000f03f"),
            true,
        );
        assert!(interpret_result.is_ok());

        let result = runner.stack.pop().unwrap();
        assert_eq!(
            result,
            pad_left(&_hex_string_to_bytes(
                "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470"
            ))
        );
    }

    #[test]
    fn test_blockhash() {
        // TODO: test with a fork
    }

    #[test]
    fn test_coinbase() {
        let mut runner = Runner::_default();
        coinbase(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xc0; 20]));
    }

    #[test]
    fn test_timestamp() {
        let mut runner = Runner::_default();
        timestamp(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0u8; 32]));
    }

    #[test]
    fn test_number() {
        // TODO: test with a fork
        let mut runner = Runner::_default();
        number(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xff; 4]));
    }

    #[test]
    fn test_difficulty() {
        // TODO: test with a fork
        let mut runner = Runner::_default();
        difficulty(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x45; 8]));
    }

    #[test]
    fn test_gaslimit() {
        let mut runner = Runner::_default();
        gaslimit(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x01, 0xC9, 0xC3, 0x80]));
    }

    #[test]
    fn test_chainid() {
        let mut runner = Runner::_default();
        chainid(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x01]));
    }

    #[test]
    fn test_selfbalance() {
        let mut runner = Runner::_default();
        selfbalance(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x00]));

        // transfer 100 wei to the contract
        let _ = runner
            .state
            .transfer(runner.caller, runner.address, pad_left(&[0x64]));
        selfbalance(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x64]));
    }

    #[test]
    fn test_basefee() {
        let mut runner = Runner::_default();
        basefee(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x0a]));
    }
}
