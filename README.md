
# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a command line or in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.

![Github action](https://github.com/Yashiru/evm-rs-emulator/workflows/CI/badge.svg)

***
<details>
  <summary>
    <h2>Rust crate</h2>
    <p>👉 Use the evm-rs-emulator crate in your rust project.</p>
  </summary>

  ## Install crate
  ```bash
  cargo add evm-rs-emulator
  ```

  ## Usage
  ```rust
  use evm_rs_emulator::Runner;

  fn main() {
    let caller = [
      0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
      0x00, 0x00, 0xc4, 0x11, 0xe8,
    ];
    let origin: Option<[u8; 20]> = None;
    let address: Option<[u8; 20]> = None;
    let value: Option<[u8; 32]> = None;
    let data: Option<Vec<u8>> = None;
    let bytecode: Vec<u8> = vec![0x60, 0xff, 0x60, 0xff];
    
    // Create a new interpreter
    let mut runner =
        Runner::new(caller, origin, address, value, data, None);

    // Run all the bytecode
    let _ = interpreter.interpret(bytecode, Some(255), true);

    // Or run the bytecode OPCODE by OPCODE
    runner.bytecode = bytecode;
    runner.debug_level = Some(255);
    // Run the first 3 opcodes
    let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
    let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
    let _ = runner.interpret_op_code(runner.bytecode[runner.pc]);
  }
  ```
</details>



## 🚧 Warning 🚧
This project is currently experimental and subject to frequent changes as we are still working on stabilizing EVM emulation.
It has not been audited for security purposes and should not be used in production yet.

### Missing features (Contributions welcome ❤️)
- [ ] EVM gas usage

## Contributions

To contribute to the EVM Rust Emulator, you will need to have Rust and Cargo installed on your system. 
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Once you have these tools installed, you can clone the project.
```bash
git clone git@github.com:nick199910/evm-simulation.git
```

To run the tests, you can use the following command.
```bash
cargo test
```

## License

The underlying source code is free and unencumbered software released into the public domain. Check LICENSE file for more information.
