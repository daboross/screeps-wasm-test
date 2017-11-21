const Pbf = require('pbf');

const {RoomPosition, StructureSpawn, AllOwnedSpawns} = require('protobuf');

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

    static new(code) {
        let module = new WebAssembly.Module(code);

        let holder = {
            instance: null
        };
        let wasm_module_instance = new WebAssembly.Instance(module, {
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

function main() {
    let cpu1 = Game.cpu.getUsed();

    let inner_result = 0;
    for (var i = 0; i < 20000; i++) {
        inner_result += i * 7 * (i - 2) * (i - 3);
    }
    console.log(inner_result);

    let cpu2 = Game.cpu.getUsed();

    let code = require("screeps_exports_b64");

    let cpu3 = Game.cpu.getUsed();

    let instance = ScreepsWasm.new(code);

    let cpu4 = Game.cpu.getUsed();

    instance.debug_room_pos_in_rust({x: 20, y: 40, roomName: "Hello world!"});

    let cpu5 = Game.cpu.getUsed();

    console.log(`cpu: [${cpu1}, ${cpu2}, ${cpu3}, ${cpu4}, ${cpu5}].`);
    console.log(`warmup took: ${cpu2 - cpu1}`);
    console.log(`requiring wasm binary took: ${cpu3 - cpu2}`)
    console.log(`loading wasm module took: ${cpu4 - cpu3}`);
    console.log(`running simple callback took: ${cpu5 - cpu4}`);
}

module.exports.loop = main;
