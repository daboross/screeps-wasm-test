#[macro_use]
extern crate wasm_protobuf_gen;

mod hi {
    pub fn asdf() -> &'static str {
        "hi"
    }
}

js_fn! {
    fn hi(input: &[u8]) => { println!("hi"); };
    fn bye(x: &(([((u8))]))) => hi::asdf;
}

#[test]
fn printit() {
    panic!("\n\n{}\n\n{}\n\n", hi(), bye());
}
