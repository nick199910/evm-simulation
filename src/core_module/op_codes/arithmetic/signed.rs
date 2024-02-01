use crate::core_module::runner::Runner;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::I256;
use ethers::types::U256;

pub fn sdiv(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1: [u8; 32] = runner.stack.pop()?;
    let pop2: [u8; 32] = runner.stack.pop()?;

    let a: I256 = I256::from_raw(U256::from_big_endian(&pop1));
    let b: I256 = I256::from_raw(U256::from_big_endian(&pop2));

    let result = a.checked_div(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(I256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn smodulo(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1: [u8; 32] = runner.stack.pop()?;
    let pop2: [u8; 32] = runner.stack.pop()?;

    let a: I256 = I256::from_raw(U256::from_big_endian(&pop1));
    let b: I256 = I256::from_raw(U256::from_big_endian(&pop2));

    let result = a.checked_rem(b);

    let mut result_bytes = [0u8; 32];
    result
        .unwrap_or(I256::from(0))
        .to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}
