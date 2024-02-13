mod core_module;
use core_module::state::EvmState;
use core_module::utils::errors::ExecutionError;
use std::{env, fs};

// Colored output
use colored::*;

fn main() -> Result<(), ExecutionError> {
    let mut caller = [
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xe8,
    ];
    let mut origin: Option<[u8; 20]> = None;
    let mut address: Option<[u8; 20]> = Some([
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0xc4, 0x11, 0xee,
    ]);
    let mut value: Option<[u8; 32]> = None;
    let mut data: Option<Vec<u8>> = None;
    let mut bytecode: String = Default::default();
    let state: EvmState;
    let mut debug_level: Option<u8> = Some(255);

    state = EvmState::new(None);

    // Create a new interpreter
    let mut interpreter =
        core_module::runner::Runner::new(caller, origin, address, value, data, Some(state), None);

    // Check if bytecode is an hex value of a file path
    if bytecode.starts_with("0x") {
        let bytecode = hex::decode(&bytecode[2..]).expect("Invalid bytecode");

        // Interpret the bytecode
        let _ = interpreter.interpret(bytecode, true);
        return Ok(());
    }

    Ok(())
}
