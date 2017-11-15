#!/usr/bin/env node
const fs = require('fs');
const Pbf = require('./pbf.js');

const {RoomPosition} = require('./protobuf/pos.js');

class ScreepsWasm {
    constructor(wasm_module) {
        this.module = wasm_module;
        this._mem = new Uint8Array(wasm_module.exports["memory"].buffer);
        this._functions = {
            allocate_uninitialized_bytes: wasm_module.exports["allocate_uninitialized_bytes"],
            allocate_zeroed_bytes: wasm_module.exports["allocate_zeroed_bytes"],
            deallocate_bytes: wasm_module.exports["deallocate_bytes"],
            deallocate_uninitialized_bytes: wasm_module.exports["deallocate_uninitialized_bytes"],
            debug_room_position: wasm_module.exports["debug_room_position"],
            debug_room_position_consuming: wasm_module.exports["debug_room_position_consuming"],
        }
    }

    static async new(code) {
        let module = await WebAssembly.compile(code);

        let holder = {
            instance: null
        };
        let wasm_module_instance = await WebAssembly.instantiate(module, {
            env: {
                print_str(ptr, len) {
                    let view = holder.instance._mem.subarray(ptr, ptr + len);
                    let result = "";
                    for (var i = 0; i < view.length; i++) {
                        result += String.fromCharCode(view[i]);
                    }
                    console.log(result);
                }
            }
        });

        let screeps_wasm = new ScreepsWasm(wasm_module_instance);
        holder.instance = screeps_wasm;

        return screeps_wasm;
    }

    allocate_zeroed(len) {
        let ptr = this._functions.allocate_zeroed_bytes(len);
        return [ptr, this._mem.subarray(ptr, ptr + len)];
    }

    allocate_uninitialized(len) {
        let ptr = this._functions.allocate_uninitialized_bytes(len);
        return [ptr, this._mem.subarray(ptr, ptr + len)];
    }

    deallocate(ptr, len) {
        this._functions.deallocate_zeroed(ptr, len);
    }

    deallocate_uninitialized(ptr, len, cap) {
        this._functions.deallocate_uninitialized_bytes(ptr, len, cap);
    }

    allocate_array(arr) {
        let len = arr.length;
        let [ptr, view] = this.allocate_uninitialized(len);
        view.set(arr);
        return [ptr, len];
    }

    allocate_room_position(room_position) {
        let pbf = new Pbf();
        pbf.writeRawMessage(RoomPosition.write, room_position);
        let buffer = pbf.finish();
        return this.allocate_array(buffer);
    }

    debug_room_pos_in_rust(pos) {
        let [ptr, len] = this.allocate_room_position(pos);
        return this._functions.debug_room_position_consuming(ptr, len);
    }
}

async function main() {
    let code = fs.readFileSync("target/wasm32-unknown-unknown/release/screeps-exports.wasm");

    let instance = await ScreepsWasm.new(code);

    instance.debug_room_pos_in_rust({x: 20, y: 40, roomName: "Hello world!"});
}

main()
.catch(err => { console.log(err); process.exit(1); });
