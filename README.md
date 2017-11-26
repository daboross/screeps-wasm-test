Rust WASM tests with the intention to use in Screeps
==

### Setup

This is based off of the bare-wasm target in nightly rust. You'll need a nightly
2017-11-25 or newer.

To setup:
- Install rust using [rustup](https://www.rustup.rs/)
- Add/update nightly toolchain:

  ```
  rustup update nightly
  ```
- Add wasm32 target:

  ```
  rustup target add --toolchain nightly wasm32-unknown-unknown
  ```
- To run locally, install nodeJS version 8 or higher (https://nodejs.org/en/)
- Clone this project
- Optionally install:
  - [wasm-gc](https://github.com/alexcrichton/wasm-gc):
    useful for trimming excess code from .wasm files


    ```
    cargo install --git https://github.com/alexcrichton/wasm-gc
    ```
  - [wabt](https://github.com/WebAssembly/wabt):
    useful for turning .wasm files into readable / debugable .wat text files

    ```
    git clone https://github.com/WebAssembly/wabt
    cd wabt
    make # your opts here (see wabt repository)
    ```


### Running it!


```
# build with Cargo
cargo build --release --target=wasm32-unknown-unknown
# optionally reduce WASM code size
wasm-gc target/wasm32-unknown-unknown/release/*.wasm target/wasm32-unknown-unknown/release/*.wasm
# run JS side
node ./src/javascript/callit.js
```
