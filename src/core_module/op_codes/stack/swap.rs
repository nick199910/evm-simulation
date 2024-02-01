use crate::core_module::runner::Runner;
use crate::core_module::utils::errors::ExecutionError;

pub fn swap1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(1);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(2);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(3);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(4);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap5(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(5);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap6(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(6);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap7(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(7);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn swap8(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(8);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap9(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(9);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap10(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(10);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap11(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(11);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap12(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(12);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap13(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(13);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap14(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(14);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap15(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(15);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}

pub fn swap16(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.swap(16);

    if result.is_err() {
        return Err(result.unwrap_err());
    }
    // Increment PC
    runner.increment_pc(1)
}
