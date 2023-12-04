# RTC-Network

## UseAge

```bash
# Install dependencies
sudo apt install --assume-yes git clang curl libssl-dev llvm libudev-dev make protobuf-compiler
# Install
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
# Configure
source ~/.cargo/env
```

## Build

```bash
cargo build --release
```

## Start Node

```bash
./target/release/node-template purge-chain
./target/release/node-template --dev
RUST_BACKTRACE=1 ./target/release/node-template -ldebug --dev
```

## Connect Node

`https://polkadot.js.org/apps/#/explorer?rpc=ws://localhost:9944`

## doc

```bash
cargo doc --open
cd ./target/doc/
caddy file-server --listen :1025
```

open browser jump to: localhost:1025/help.html
