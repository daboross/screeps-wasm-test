#!/usr/bin/env node

const fs = require('fs');
const Pbf = require('./pbf.js');
const {RoomPosition} = require('./protobuf/pos.js');

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

    console.log("pos x: " + instance.room_pos_x_in_rust({x: 15, y: 30, roomName: "hello!"}))
}

class WasmWrapper {
    constructor(instance) {
        this.module = instance;
        this._alloc = this.module.exports["alloc_bytes"];
        this._dealloc = this.module.exports["dealloc_bytes"];
        this._sum = this.module.exports["sum"];
        this._retrieve_x = this.module.exports["retrieve_x"];
        this._mem = new Uint8Array(this.module.exports["memory"].buffer);
    }

    static async new(code) {
        let module = await WebAssembly.compile(code);
        let memory_holder = {
            memory: null,
        };
        let instance = await WebAssembly.instantiate(module, {
            env: {
                callback(x) {
                    return x;
                },
                print_bytes(ptr, len) {
                    let view = memory_holder.memory.subarray(ptr, ptr + len);
                    console.log("[DEBUG] bytes: " + view);
                },
                print_str(ptr, len) {
                    let view = memory_holder.memory.subarray(ptr, ptr + len);
                    let result = [];
                    for (var i = 0; i < view.length; i++) {
                        // TODO: fix incorrect utf8 decoding or ensure ascii on the rust side
                        result.push(String.fromCharCode(view[i]));
                    }
                    console.log("[DEBUG] str: " + result.join(''));
                }
            }
        });
        let wrapper = new WasmWrapper(instance);
        memory_holder.memory = wrapper._mem; // give access to print_bytes, etc.

        return wrapper;
    }

    alloc(len) {
        let ptr = this._alloc(len);
        return [ptr, this._mem.subarray(ptr, ptr + len)];
    }

    dealloc(ptr, len) {
        this._dealloc(ptr, len);
    }

    store_to_ptr_len(input_arraybuffer) {
        let len = input_arraybuffer.length;
        let [ptr, view] = this.alloc(len);
        view.set(input_arraybuffer);
        console.log("stored: " + view + " : len: " + len);
        return [ptr, len];
    }

    sum(ptr, len) {
        return this._sum(ptr, len);
    }

    retrieve_x(ptr, len) {
        return this._retrieve_x(ptr, len);
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

    send_pos_to_ptr_len(room_position) {
        let pbf = new Pbf();
        pbf.writeRawMessage(RoomPosition.write, room_position);
        let buffer = pbf.finish();
        return this.store_to_ptr_len(buffer);
    }

    room_pos_x_in_rust(pos) {
        let [ptr, len] = this.send_pos_to_ptr_len(pos);
        let val = this.retrieve_x(ptr, len);
        this.dealloc(ptr, len);
        return val;
    }
}

main()
.catch(err => { console.log(err); process.exit(1); });
