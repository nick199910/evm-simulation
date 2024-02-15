use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

// Primitive types
use crate::core_module::utils::bytes::{u64_to_u256_array, u64_x4_array_to_u8_x32_array};
use ethers::types::U256;
use ethers::utils::keccak256;

pub fn not(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);

    let result = !a;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn xor(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a ^ b;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn or(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a | b;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn and(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = a & b;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn shl(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = b << a;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn shr(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let a = U256::from_big_endian(&pop1);
    let b = U256::from_big_endian(&pop2);

    let result = b >> a;

    let mut result_bytes = [0u8; 32];
    result.to_big_endian(&mut result_bytes);

    let result = runner.stack.push(result_bytes);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn sha3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let offset = U256::from_big_endian(&pop1).as_usize();
    let size = U256::from_big_endian(&pop2).as_usize();

    let data_to_hash = unsafe { runner.memory.read(offset, size) };

    if data_to_hash.is_err() {
        return Err(data_to_hash.unwrap_err());
    }

    let bytes = keccak256(&data_to_hash?);

    runner.stack.push(bytes)?;

    // Increment PC
    runner.increment_pc(1)
}

// x (b+1) * 8 -> x 256
pub fn signextend(runner: &mut Runner) -> Result<(), ExecutionError> {
    let pop1 = runner.stack.pop()?;
    let pop2 = runner.stack.pop()?;

    let x = U256::from_big_endian(&pop1);
    let y = U256::from_big_endian(&pop2);

    if x > U256::from(31) {
        runner.stack.push(pop2)?;
    } else {
        // 符号位是第几位
        let t = U256::from(256) - U256::from(8) * (x + U256::from(1));

        // 8x + 7
        let bit_index = U256::from(255) - t;

        // 0 是正数 1是负数
        let bit = y.bit(bit_index.as_usize());

        let mask = (U256::from(1) << bit_index) - U256::from(1);

        let result = if bit { y | !mask } else { y & mask };

        let ret = u64_x4_array_to_u8_x32_array(result);

        runner.stack.push(ret)?;
    }
    // Increment PC
    runner.increment_pc(1)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core_module::utils::bytes::pad_left_one;
    use crate::core_module::{op_codes::memory::mstore, utils::bytes::pad_left};

    #[test]
    fn test_not() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));

        not(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff, 0xff,
            0xff, 0xff, 0xff, 0xfb,
        ]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_xor() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x08]));

        xor(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0x04 ^ 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn or_test() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x08]));

        or(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0x04 | 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_and() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x08]));

        and(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0x04 & 0x08]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_shl() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x02]));

        shl(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0x10]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_shr() {
        let mut runner = Runner::_default();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x02]));

        shr(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left(&[0x01]);

        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    fn test_sha256() {
        let mut runner = Runner::_default();

        let _ = runner.stack.push(pad_left(&[0xff, 0xff, 0xff, 0xff]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        mstore(&mut runner).unwrap();
        let _ = runner.stack.push(pad_left(&[0x04]));
        let _ = runner.stack.push(pad_left(&[0x00]));
        sha3(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_output = [
            0xe8, 0xe7, 0x76, 0x26, 0x58, 0x6f, 0x73, 0xb9, 0x55, 0x36, 0x4c, 0x7b, 0x4b, 0xbf,
            0x0b, 0xb7, 0xf7, 0x68, 0x5e, 0xbd, 0x40, 0xe8, 0x52, 0xb1, 0x64, 0x63, 0x3a, 0x4a,
            0xcb, 0xd3, 0x24, 0x4c,
        ];

        assert_eq!(result, expected_output);
        assert_eq!(runner.stack.stack.len(), 0);
    }

    #[test]
    pub fn test_signextend() -> Result<(), ExecutionError> {
        let mut runner = Runner::_default();

        let _ = runner.stack.push(pad_left(&[0xff]));
        let _ = runner.stack.push(pad_left(&[0x00]));

        signextend(&mut runner).unwrap();

        let result = runner.stack.pop().unwrap();
        let expected_result = pad_left_one(&[0xff]);
        assert_eq!(result, expected_result);
        assert_eq!(runner.stack.stack.len(), 0);

        let _ = runner.stack.push(pad_left(&[
            0x0b, 0xb8, 0x00, 0x04, 0xff, 0xff, 0xff, 0xff, 0x00, 0x00, 0x00, 0x00, 0x05, 0x17,
            0xb5, 0xcf, 0x78, 0xea, 0x82, 0x54,
        ]));
        let _ = runner.stack.push(pad_left(&[0x0b]));
        signextend(&mut runner).unwrap();
        let result = runner.stack.pop().unwrap();

        // let hex: String =
        //     utils::debug::to_hex_string(result.as_slice().try_into().unwrap());
        // println!("{}", hex);
        // let expected_result = pad_left_one(&[0xff]);
        // assert_eq!(result, expected_result);
        // assert_eq!(runner.stack.stack.len(), 0);
        Ok(())
    }
}
