#!/usr/bin/env node
const fs = require('fs');
const Pbf = require('./pbf.js');

const {RoomPosition} = require('./protobuf/pos.js');

async function main() {
    let code = fs.readFileSync("target/wasm32-unknown-unknown/release/screeps-wasm-test.wasm").buffer;

    let instance = await WasmWrapper.new(code);

    instance.debug_room_pos_in_rust({x: 20, y: 40, roomName: "Hello world!"});
}

class WasmWrapper {
    constructor(instance) {
        this.module = instance;
        this._alloc = this.module.exports["alloc_bytes"];
        this._dealloc = this.module.exports["dealloc_bytes"];
        this._debug_room_position = this.module.exports["debug_room_position"];
        this._mem = new Uint8Array(this.module.exports["memory"].buffer);
    }

    static async new(code) {
        let module = await WebAssembly.compile(code);
        let holder = {
            i: null,
        };
        let module_instance = await WebAssembly.instantiate(module, {
            env: {
                callback(x) {
                    return x;
                },
                print_bytes(ptr, len) {
                    let mem = holder.i._mem;
                    let view = mem.subarray(ptr, ptr + len);
                    console.log("[DEBUG] bytes: " + view);
                },
                print_str(ptr, len) {
                    let mem = holder.i._mem;
                    let view = mem.subarray(ptr, ptr + len);
                    let result = [];
                    for (var i = 0; i < view.length; i++) {
                        // TODO: fix incorrect utf8 decoding or ensure ascii on the rust side
                        result.push(String.fromCharCode(view[i]));
                    }
                    console.log("[DEBUG] str: " + result.join(''));
                }
            }
        });

        let wrapper_instance = new WasmWrapper(module_instance);

        holder.i = wrapper_instance;

        return wrapper_instance;
    }

    alloc(len) {
        let ptr = this._alloc(len);
        return [ptr, this._mem.subarray(ptr, ptr + len)];
    }

    dealloc(ptr, len) {
        this._dealloc(ptr, len);
    }

    store_to_ptr_len(arr) {
        let len = arr.length;
        let [ptr, view] = this.alloc(len);
        view.set(arr);
        return [ptr, len];
    }

    debug_sent_room_position(ptr, len) {
        return this._debug_room_position(ptr, len);
    }

    send_pos_to_ptr_len(room_position) {
        let pbf = new Pbf();
        pbf.writeRawMessage(RoomPosition.write, room_position);
        let buffer = pbf.finish();
        return this.store_to_ptr_len(buffer);
    }

    debug_room_pos_in_rust(pos) {
        let [ptr, len] = this.send_pos_to_ptr_len(pos);
        let val = this.debug_sent_room_position(ptr, len);
        this.dealloc(ptr, len);
        return val;
    }
}

main()
.catch(err => { console.log(err); process.exit(1); });
