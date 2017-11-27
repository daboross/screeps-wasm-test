extern crate wasm_protobuf_gen_build;

fn main() {
    match wasm_protobuf_gen_build::translate_files(
        "tests/code_is_called.rs",
        "./out.js",
        "WasmTesting",
    ) {
        Ok(()) => (),
        Err(e) => panic!("error: {}", e),
    }
}
