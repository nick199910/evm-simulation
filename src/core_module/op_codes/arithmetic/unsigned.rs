use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

pub fn add(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_add(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn mul(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_mul(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sub(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_sub(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn modulo(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a.checked_rem(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn div(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a.checked_div(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn addmod(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;
    let pop3 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);
    let c = U256::from_big_endian(&pop3);

    let (result, _) = a.overflowing_add(b);
    let result = result.checked_rem(c);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn mulmod(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;
    let pop3 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);
    let c = U256::from_big_endian(&pop3);

    let (result, _) = a.overflowing_mul(b);
    let result = result.checked_rem(c);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(U256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn exp(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let (result, _) = a.overflowing_pow(b);

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}
