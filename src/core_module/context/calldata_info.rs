use crate::core_module::context::evm_context::EvmContext;

#[derive(Debug, Clone)]
pub struct CallDataInfo {
    pub origin: Vec<u8>,
    pub new: Vec<u8>,
    pub replace: Vec<Vec<u8>>
}

impl CallDataInfo {
    pub fn new() -> Self {
        Self {
            origin: vec![],
            new: vec![],
            replace: vec![],
        }
    }
}