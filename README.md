Rust WASM tests with the intention to use in Screeps
==

### Setup

This is based off of WIP work in https://github.com/rust-lang/rust/pull/45905.

You'll need to:

- Install regular (nightly) rust using [rustup](https://www.rustup.rs/)
- Clone https://github.com/alexcrichton/rust with the 'add-wasm-target' branch
- Build said clone with instructions from https://github.com/rust-lang/rust/pull/45905#issue-272863447
- Link the toolchain to your rustup with

  ```
  rustup toolchain link wasm32 build/<your OS tripple here>/stage2
  ```

  For example, I ran `rustup toolchain link wasm32 build/x86_64-unknown-linux-gnu/stage2`.
- Install nodeJS version 8 or higher (https://nodejs.org/en/)
- Clone this project
- Optionally install:
  - [wasm-gc](https://github.com/alexcrichton/wasm-gc):

    ```
    cargo install --git https://github.com/alexcrichton/wasm-gc
    ```
  - [wabt](https://github.com/WebAssembly/wabt)

    ```
    git clone https://github.com/WebAssembly/wabt
    cd wabt
    make # your opts here (see wabt repository)
    ```


### Running it!


```
# build with Cargo
cargo +wasm32 build --release --target=wasm32-unknown-unknown
# optionally reduce WASM code size
wasm-gc target/wasm32-unknown-unknown/release/screeps-wasm-test.wasm target/wasm32-unknown-unknown/release/screeps-wasm-test.wasm
# run JS side
node ./src/javascript/callit.js
```
