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

    console.log("sum: " + instance.sum_in_rust([1, 2, 3, 4, 5]));
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

    sum_in_rust(arr) {
        let len = arr.length;
        let [ptr, view] = this.alloc(arr.length);
        for (var i = 0; i < len; i++) {
            view[i] = arr[i];
        }
        let val = this.sum(ptr, len);
        this.dealloc(ptr, len);
        return val;
    }
}

main()
.catch(err => { console.log(err); process.exit(1); });
