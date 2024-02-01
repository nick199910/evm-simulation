use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

pub fn pop(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.pop();

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}
