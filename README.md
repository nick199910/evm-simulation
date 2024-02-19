
# EVM Rust Emulator

The EVM Rust Emulator is a simple in-memory Ethereum Virtual Machine (EVM) emulator written in Rust. It is designed to be a lightweight and easy-to-use tool for developers who want to test EVM bytecode execution directly in a command line or in a Rust crate, without using a full EVM node with his RPC to interact with a blockchain.


## 🚧 Warning 🚧
This project is currently experimental and subject to frequent changes as we are still working on stabilizing EVM emulation.
It has not been audited for security purposes and should not be used in production yet.

### Upcoming features (Contributions welcome ❤️)
- [ ] EVM gas usage
- [ ] EVM concurrent exec 
- [ ] EVM instructor concurrent exec 
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
