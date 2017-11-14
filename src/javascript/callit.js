#!/usr/bin/env node

const fs = require('fs');

async function main() {
    let code = fs.readFileSync("target/wasm32-unknown-unknown/release/screeps-wasm-test.wasm");

    // let wasm_module = await WebAssembly.compile(code.buffer);

    // console.log(WebAssembly.Module.imports(wasm_module));

    // let instance = await WebAssembly.instantiate(wasm_module, {
    //     env: {
    //         callback: (x => x + 10),
    //     }
    // });

    // console.log(instance.exports["entry"]());
    // for (var i = 0; i < 20; i++) {
    //     console.log(instance.exports["fib"](i));
    // }

    let instance = await WasmWrapper.new(code);

    let [ptr, mem] = instance.alloc(10);
    console.log("ptr: " + ptr);
    for (var i = 0; i < mem.length; i++) {
        mem[i] = 20 + i;
    }
    console.log("mem: " + mem);
    console.log("sum: " + instance.sum(ptr, 10));
    instance.dealloc(10, ptr);
}

class WasmWrapper {
    constructor(instance) {
        this.module = instance;
        this._alloc = this.module.exports["alloc_bytes"];
        this._dealloc = this.module.exports["dealloc_bytes"];
        this._sum = this.module.exports["sum"];
        this._mem = new Uint8Array(this.module.exports["memory"].buffer);
    }
    static async new(code) {
        let module = await WebAssembly.compile(code);
        let instance = await WebAssembly.instantiate(module, {
            env: {
                callback: (x => x),
            }
        });
        return new WasmWrapper(instance);
    }
    alloc(len) {
        let ptr = this._alloc(len);
        return [ptr, this._mem.subarray(ptr, ptr + len)];
    }
    dealloc(ptr, len) {
        this._dealloc(ptr, len);
    }
    sum(ptr, len) {
        return this._sum(ptr, len);
    }
}

main()
.catch(err => { console.log(err); process.exit(1); });
