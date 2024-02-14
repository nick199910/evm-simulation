use crate::core_module::memory::Memory;
use crate::core_module::stack::Stack;
use crate::core_module::state::EvmState;

#[derive(Debug)]
pub struct OpCodeContext<'a> {
    pub pc: u64,

    pub op_code: &'a str,

    // Data
    pub state: EvmState,

    pub memory: Memory,

    pub calldata: Memory,

    pub returndata: Memory,

    pub stack: Stack,
}
