use crate::core_module::runner::Runner;
use crate::core_module::state::Log;

use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use ethers::types::U256;

pub fn log0(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size = U256::from_big_endian(&runner.stack.pop()?);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    // Increment PC
    runner.increment_pc(1)
}

pub fn log1(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    // Increment PC
    runner.increment_pc(1)
}

pub fn log2(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    // Increment PC
    runner.increment_pc(1)
}

pub fn log3(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }
    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2, topic3],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    // Increment PC
    runner.increment_pc(1)
}

pub fn log4(runner: &mut Runner) -> Result<(), ExecutionError> {
    // Check if static mode is enabled
    if runner.state.static_mode {
        return Err(ExecutionError::StaticCallStateChanged);
    }

    let offset = U256::from_big_endian(&runner.stack.pop()?);
    let size: U256 = U256::from_big_endian(&runner.stack.pop()?);

    let raw_topic1: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic1 = [0u8; 32];
    raw_topic1.to_big_endian(&mut topic1);

    let raw_topic2: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic2 = [0u8; 32];
    raw_topic2.to_big_endian(&mut topic2);

    let raw_topic3: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic3 = [0u8; 32];
    raw_topic3.to_big_endian(&mut topic3);

    let raw_topic4: U256 = U256::from_big_endian(&runner.stack.pop()?);
    let mut topic4 = [0u8; 32];
    raw_topic4.to_big_endian(&mut topic4);

    let log_data = unsafe { runner.memory.read(offset.as_usize(), size.as_usize())? };

    let log = Log {
        address: runner.address,
        topics: vec![topic1, topic2, topic3, topic4],
        data: log_data.clone(),
    };

    runner.state.logs.push(log);

    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use crate::core_module::runner::Runner;
    use crate::core_module::utils::bytes::{_hex_string_to_bytes, pad_left};
    use crate::core_module::utils::errors::ExecutionError;

    #[test]
    fn test_log0() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("604260005260206000a0"), true);
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 0);
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log1() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("604260005260ff60206000a1"), true);
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 1);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log2() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> =
            runner.interpret(_hex_string_to_bytes("6042600052606060ff60206000a2"), true);
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 2);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log3() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("604260005260ac606060ff60206000a3"),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 3);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.topics[2] == pad_left(&[0xac]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }

    #[test]
    fn test_log4() {
        let mut runner = Runner::_default();
        let interpret_result: Result<(), ExecutionError> = runner.interpret(
            _hex_string_to_bytes("6042600052601d60ac606060ff60206000a4"),
            true,
        );
        assert!(interpret_result.is_ok());

        let log = runner.state.logs.get(0).unwrap();

        assert!(log.topics.len() == 4);
        assert!(log.topics[0] == pad_left(&[0xff]));
        assert!(log.topics[1] == pad_left(&[0x60]));
        assert!(log.topics[2] == pad_left(&[0xac]));
        assert!(log.topics[3] == pad_left(&[0x1d]));
        assert!(log.address == runner.address);
        assert!(log.data == pad_left(&[0x42]));
    }
}
