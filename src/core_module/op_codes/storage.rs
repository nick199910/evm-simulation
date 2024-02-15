use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

pub fn sload(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let word = runner.state.sload(runner.address, address)?;

    let result = runner.stack.push(word);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sstore(runner: &mut Runner) -> Result<(), ExecutionError> {
    let address = runner.stack.pop()?;
    let word = runner.stack.pop()?;

    let result = runner.state.sstore(runner.address, address, word);

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
    fn test_sload() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("602e600055600054600154"), true);
        assert!(interpret_result.is_ok());

        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x00]));
        let result: [u8; 32] = runner.stack.pop().unwrap();
        assert_eq!(result, pad_left(&[0x2e]));
    }

    #[test]
    fn test_sstore() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("602e600055"), true);
        assert!(interpret_result.is_ok());

        let result = runner
            .state
            .sload(runner.address, pad_left(&[0x00]))
            .unwrap();
        assert_eq!(result, pad_left(&[0x2e]));
    }
}
