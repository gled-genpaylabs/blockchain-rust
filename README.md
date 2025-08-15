# Features

1. Basic Prototype
2. Proof-of-Work
3. Persistence and CLI`
5. Addresses
6. Transactions 2
7. Network


## Instructions

- Create wallet:
  ```sh
  cargo run createwallet
  ```
- Create blockchain:
  ```
  cargo run createblockchain <address>
  ```
- send coins (if `-m` is specified, the block will be mined immediately in the same node):
  ```
  cargo run send <from> <to> <amount> -m
  ```
- start server:
  ```
  cargo run startnode <port>
  ```
  or start miner node:
  ```
  cargo run startminer <port> <address>
  ```
- get balance:
  ```
  cargo run getbalance <address>
  ```

You can use the `RUST_LOG=info` to print the log.
