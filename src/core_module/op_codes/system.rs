use crate::core_module::runner::convert_array_to_hex;
use crate::core_module::runner::Runner;
use crate::core_module::utils::bytes;
use crate::core_module::utils::bytes::{bytes32_to_address, pad_left};
use crate::core_module::utils::environment::{
    delete_account, get_balance, get_nonce, init_account,
};
use crate::core_module::utils::errors::ExecutionError;
// Primitive types
use ethers::types::U256;

use crate::core_module::utils;
use ethers::utils::keccak256;
use revm_primitives::Address;

pub fn invalid(runner: &mut Runner) -> Result<(), ExecutionError> {
    Err(ExecutionError::InvalidOpcode(runner.bytecode[runner.pc]))
}

pub fn create(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    // caller, nonce
    let value = runner.stack.pop()?;
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size = U256::from_big_endian(&runner.stack.pop()?);
    println!("size {:?}", size);
    // Load the init code from memory
    let init_code = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };
    println!("{:?}", convert_array_to_hex(&init_code));
    // 使用Address的官方地址计算
    let nonce = get_nonce(runner.address, runner)?;
    let nonce = U256::from_big_endian(&nonce).0[0];
    let caller = &runner.caller;
    let create_address = Address::from_slice(caller).create(nonce);
    println!("{:?}", create_address);
    // Create the contract with init code as code
    init_account(*create_address.0, runner)?;
    runner.state.put_code_at(*create_address.0, init_code)?;

    let call_result = runner.call(*create_address.0, value, Vec::new(), runner.gas, false);
    // Check if the call failed
    if call_result.is_err() {
        runner.stack.push(pad_left(&[0x00]))?;
    } else {
        runner.stack.push(pad_left(&*create_address.0))?;
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner
        .state
        .put_code_at(*create_address.0, returndata.clone())?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, *create_address.0, value)?;

    // Increment PC
    runner.increment_pc(1)
}

pub fn create2(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let value = runner.stack.pop()?;
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size = U256::from_big_endian(&runner.stack.pop()?);
    let salt = runner.stack.pop()?;

    // Load the init code from memory
    let init_code = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Compute the contract address
    let init_code_hash = keccak256(init_code.clone());

    let caller = &runner.caller;
    // caller, salt, A
    let create_address = Address::from_slice(caller).create2(salt, init_code_hash);

    // Create the contract with init code as code
    init_account(*create_address.0, runner)?;
    runner.state.put_code_at(*create_address.0, init_code)?;

    // Call the contract to run its constructor
    let call_result = runner.call(*create_address.0, value, Vec::new(), runner.gas, false);

    // Check if the call failed
    if call_result.is_err() {
        runner.stack.push(pad_left(&[0x00]))?;
    } else {
        runner.stack.push(pad_left(&(*create_address.0)))?;
    }

    // Get the return data to store the real contract code
    let returndata = runner.returndata.heap.clone();
    runner.state.put_code_at(*create_address.0, returndata)?;

    // Transfer the value
    runner
        .state
        .transfer(runner.caller, *create_address.0, value)?;

    // Increment PC
    runner.increment_pc(1)
}

pub fn call(runner: &mut Runner, bypass_static: bool) -> Result<(), ExecutionError> {
    // hook_function

    // Check if static mode is enabled
    if runner.state.static_mode && !bypass_static {
        return Err(ExecutionError::StaticCallStateChanged);
    }

    // Get the values on the stack
    let gas = runner.stack.pop()?;
    let to = runner.stack.pop()?;

    let value = if bypass_static {
        [0u8; 32]
    } else {
        runner.stack.pop()?
    };

    let calldata_offset = U256::from_big_endian(&runner.stack.pop()?);
    let calldata_size = U256::from_big_endian(&runner.stack.pop()?);
    let returndata_offset = U256::from_big_endian(&runner.stack.pop()?);
    let returndata_size = U256::from_big_endian(&runner.stack.pop()?);

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        value,
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        false,
    );

    if call_result.is_err() {
        runner.stack.push(pad_left(&[0x00]))?;
    } else {
        runner.stack.push(pad_left(&[0x01]))?;
    }

    let mut return_data: Vec<u8> = runner.returndata.heap.clone();

    // Complete return data with zeros if returndata is smaller than returndata_size
    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }
    return_data = return_data[0..returndata_size.as_usize()].to_vec();

    // Write the return data to memory
    unsafe {
        runner
            .memory
            .write(returndata_offset.as_usize(), return_data)?
    };

    // Transfer the value
    if !value.eq(&[0u8; 32]) {
        runner
            .state
            .transfer(runner.address, bytes32_to_address(&to), value)?;
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn callcode(_: &mut Runner) -> Result<(), ExecutionError> {
    Err(ExecutionError::NotImplemented(0xF2))
}

pub fn delegatecall(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let gas = runner.stack.pop()?;
    let to = runner.stack.pop()?;
    let calldata_offset = U256::from_big_endian(&runner.stack.pop()?);
    let calldata_size = U256::from_big_endian(&runner.stack.pop()?);
    let returndata_offset = U256::from_big_endian(&runner.stack.pop()?);
    let returndata_size = U256::from_big_endian(&runner.stack.pop()?);

    // Load the input data from memory
    let calldata = unsafe {
        runner
            .memory
            .read(calldata_offset.as_usize(), calldata_size.as_usize())?
    };

    // Call the contract
    let call_result = runner.call(
        bytes32_to_address(&to),
        [0u8; 32],
        calldata,
        U256::from_big_endian(&gas).as_u64(),
        true,
    );

    if call_result.is_err() {
        runner.stack.push(pad_left(&[0x00]))?;
    } else {
        runner.stack.push(pad_left(&[0x01]))?;
    }

    let mut return_data: Vec<u8> = runner.returndata.heap.clone();

    // Complete return data with zeros if returndata is smaller than returndata_size
    if return_data.len() < returndata_size.as_usize() {
        return_data.extend(vec![0; returndata_size.as_usize() - return_data.len()]);
    }

    return_data = return_data[0..returndata_size.as_usize()].to_vec();

    // Write the return data to memory
    unsafe {
        runner
            .memory
            .write(returndata_offset.as_usize(), return_data)?
    };

    // Increment PC
    runner.increment_pc(1)
}

pub fn staticcall(runner: &mut Runner) -> Result<(), ExecutionError> {
    runner.state.static_mode = true;
    let result = call(runner, true);
    runner.state.static_mode = false;

    result
}

pub fn selfdestruct(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let address = runner.stack.pop()?;

    let contract_balance = get_balance(runner.address, runner)?;

    // Transfer the balance
    runner.state.transfer(
        runner.address,
        bytes32_to_address(&address),
        contract_balance,
    )?;

    // Delete the account
    delete_account(runner.address, runner)?;

    // Increment PC
    runner.increment_pc(1)
}

pub fn return_(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Get the values on the stack
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size = U256::from_big_endian(&runner.stack.pop()?);

    // Load the return data from memory
    let returndata = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    // Set the return data
    runner.returndata.heap = returndata;

    // Set the program counter to the end of the bytecode
    runner.set_pc(runner.bytecode.len());

    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, bytes32_to_address, pad_left};
    use crate::core_module::utils::environment::get_balance;
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_invalid() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("60fffe50fe60fffe"), true);
        assert!(interpret_result.is_err());

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xff]));
    }

    #[test]
    fn test_create() {
        let mut runner = Runner::_default();
        let caller = runner.caller;
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6c63ffffffff6000526004601cf3600052600d601360fff0"),
            true,
        );

        assert!(interpret_result.is_ok());

        let result = runner.stack.pop().unwrap();

        let stored_code = runner.state.get_code_at(bytes32_to_address(&result));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ffffffff"));

        let balance = get_balance(bytes32_to_address(&result), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xff]));
    }

    #[test]
    fn test_create2() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6c63ffffffff6000526004601cf360005263aaa4aaaf600d601360aff5"),
            true,
        );
        assert!(interpret_result.is_ok());

        let result = runner.stack.pop().unwrap();

        let stored_code = runner.state.get_code_at(bytes32_to_address(&result));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ffffffff"));

        let balance = get_balance(bytes32_to_address(&result), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xaf]));
    }

    #[test]
    fn test_call() {
        let mut runner = Runner::_default();
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("7067600035600757fe5b60005260086018f36000526011600f6000f0600060006000600060008561fffff1600060006020600060008661fffff1"),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_callcode() {
        let mut runner = Runner::_default();
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("f2"), true);
        assert!(interpret_result.is_err());
        assert_eq!(
            interpret_result.unwrap_err(),
            ExecutionError::NotImplemented(0xF2)
        );
    }

    #[test]
    fn test_delegatecall() {
        let mut runner = Runner::_default();
        // Create a contract that creates an exception if first slot of storage is 0
        // Call it two time with no calldata and with calldata.
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("7067600054600757fe5b60005260086018f36000526011600f6000f060006000600060008461fffff4600160005560006000602060008561fffff4"),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_staticcall() {
        let mut runner = Runner::_default();
        // Create a contract that creates an exception if first word of calldata is 0.
        // Call it two time with storage to 0 and storage to 1 (in the caller contract).
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("746b600035600b5760ff6000555b600052600c6014f36000526015600b6000f060006000600060008461fffffa60006000602060008561fffffa"),
            true,
        );
        assert!(interpret_result.is_ok());

        // Second call succeeded
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x01]));

        // First call failed
        let result = runner.stack.pop().unwrap();
        assert!(result == pad_left(&[0x00]));
    }

    #[test]
    fn test_selfdestruct() {
        let mut runner = Runner::_default();

        // Create a contract that has ff as code
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6960ff6000526001601ff3600052600a601660aaf0"),
            true,
        );
        assert!(interpret_result.is_ok());

        let address = runner.stack.pop().unwrap();

        let stored_code = runner.state.get_code_at(bytes32_to_address(&address));

        assert_eq!(stored_code.unwrap(), &_hex_string_to_bytes("ff"));

        let balance = get_balance(bytes32_to_address(&address), &mut runner).unwrap();
        assert_eq!(balance, pad_left(&[0xaa]));

        // Set the code to the new contract to CALLER SELFDESTRUCT
        let put_code_result = runner
            .state
            .put_code_at(bytes32_to_address(&address), _hex_string_to_bytes("33ff"));
        assert!(put_code_result.is_ok());

        let mut string_address = String::new();
        for &byte in bytes32_to_address(&address).iter() {
            let hh = &format!("{:02x}", byte);
            string_address.push_str(&format!("{:02x}", byte));
        }

        let tt = &string_address;

        let bytecode = format!("73{}600060006000600060008561fffff1", string_address);
        let bytecode: &str = &bytecode;

        runner.pc = 0;

        // Self destruct the contract by calling it
        let addr = runner.address;

        let selfdestruct_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes(bytecode), true);
        assert!(selfdestruct_result.is_ok());

        let result = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x01]));

        let stored_code = runner.state.get_code_at(bytes32_to_address(&address));
        assert!(stored_code.is_none());

        let balance_result = get_balance(bytes32_to_address(&result), &mut runner);
        assert!(balance_result.is_err());
        assert_eq!(balance_result.unwrap_err(), ExecutionError::AccountNotFound);

        let receiver_balance = get_balance(runner.address, &mut runner).unwrap();
        assert_eq!(receiver_balance, balance);
    }
}
