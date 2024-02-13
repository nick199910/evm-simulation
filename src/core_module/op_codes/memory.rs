use crate::core_module::runner::Runner;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

pub fn mload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = U256::from_big_endian(&runner.stack.pop()?);
    let word = unsafe { runner.memory.mload(address.as_usize())? };
    let result = runner.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn mstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = U256::from_big_endian(&runner.stack.pop()?);
    let data = runner.stack.pop()?;

    let result = unsafe { runner.memory.mstore(address.as_usize(), data) };

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn msize(runner: &mut Runner) -> Result<(), ExecutionError> {
    let mut bytes_msize = [0u8; 32];
    U256::from(runner.memory.msize() as u64).to_big_endian(&mut bytes_msize);

    let result = runner.stack.push(bytes_msize);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, pad_left};
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_mload() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(_hex_string_to_bytes("7f00000000000000000000000000000000000000000000000000000000000000ff600052600051600151"), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0xff, 0x00]));
    }

    #[test]
    fn test_mstore() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes(
                "7fffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff600052",
            ),
            true,
        );
        assert!(interpret_result.is_ok());

        let memory_content = unsafe { runner.memory.read(0x00, 0x20) };
        assert!(memory_content.is_ok());

        assert_eq!(memory_content.unwrap(), [0xff; 32]);
    }

    #[test]
    fn test_msize() {
        let mut runner = Runner::_default(3);
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("5960005150596039515059"), true);
        assert!(interpret_result.is_ok());

        let result1: [u8; 32] = runner.stack.pop().unwrap();
        let result2: [u8; 32] = runner.stack.pop().unwrap();
        let result3: [u8; 32] = runner.stack.pop().unwrap();

        assert_eq!(result1, pad_left(&[0x60]));
        assert_eq!(result2, pad_left(&[0x20]));
        assert_eq!(result3, pad_left(&[0x00]));
    }
}
