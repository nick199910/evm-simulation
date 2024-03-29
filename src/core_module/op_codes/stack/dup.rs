use crate::core_module::runner::Runner;
use crate::core_module::utils;
use crate::core_module::utils::errors::ExecutionError;

pub fn dup1(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(1);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup2(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(2);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup3(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(3);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup4(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(4);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup5(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(5);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup6(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(6);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup7(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(7);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup8(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(8);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup9(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(9);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup10(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(10);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup11(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(11);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup12(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(12);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup13(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(13);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup14(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(14);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup15(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(15);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

pub fn dup16(runner: &mut Runner) -> Result<(), ExecutionError> {
    let result = runner.stack.dup(16);

    if result.is_err() {
        return Err(result.unwrap_err());
    }

    // Increment PC
    runner.increment_pc(1)
}

/* -------------------------------------------------------------------------- */
/*                                    TESTS                                   */
/* -------------------------------------------------------------------------- */

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_dup1() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(vec![0x60, 0xff, 0x80], true);
        assert_eq!(runner.stack.stack.len(), 2);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup2() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x81], true);
        assert_eq!(runner.stack.stack.len(), 3);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup3() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x82], true);
        assert_eq!(runner.stack.stack.len(), 4);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup4() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x83],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 5);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup5() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x84,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 6);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup6() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x85,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 7);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup7() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x86,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 8);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup8() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x87,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 9);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup9() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x88,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 10);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup10() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x89,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 11);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup11() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8a,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 12);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup12() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8b,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 13);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup13() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x8c,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 14);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup14() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x8d,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 15);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup15() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x8e,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 16);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }

    #[test]
    fn test_dup16() {
        let mut runner = Runner::_default();

        let _ = runner.interpret(
            vec![
                0x60, 0xff, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01, 0x60, 0x01,
                0x60, 0x01, 0x60, 0x01, 0x8f,
            ],
            true,
        );
        assert_eq!(runner.stack.stack.len(), 17);
        assert_eq!(runner.stack.stack.first(), runner.stack.stack.last());
    }
}
