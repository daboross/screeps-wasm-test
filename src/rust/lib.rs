extern crate quick_protobuf;

mod protobuf;
mod external_allocations;
mod decoding;
mod utils;
mod functionality;

#[no_mangle]
pub unsafe extern "C" fn allocate_uninitialized_bytes(len: usize) -> *mut u8 {
    external_allocations::alloc_bytes_uninitialized(len)
}

#[no_mangle]
pub unsafe extern "C" fn allocate_zeroed_bytes(len: usize) -> *mut u8 {
    external_allocations::alloc_bytes_zeroed(len)
}

#[no_mangle]
pub unsafe extern "C" fn deallocate_bytes(ptr: *mut u8, len: usize) {
    external_allocations::deallocate_exact(ptr, len)
}

#[no_mangle]
pub unsafe extern "C" fn operate_world(ptr: *mut u8, len: usize) {
    let input = decoding::read_ptr(ptr, len);

    functionality::operate_world(input).expect("TODO: output useful error here");
}

#[no_mangle]
pub unsafe extern "C" fn operate_world_consuming(ptr: *mut u8, len: usize) {
    let input = decoding::consume_ptr(ptr, len);

    functionality::operate_world(&input).expect("TODO: output useful error here");
}

mod imports {
    extern "C" {
        pub fn print_str(ptr: *mut u8, len: usize);

        pub fn execute_spawn_spawn_creep(execution_ptr: *const u8, execution_len: usize) -> i32;

        pub fn execute_creep_move_to(
            creep_name_ptr: *const u8,
            creep_name_len: usize,
            serialized_position_ptr: *const u8,
            serialized_position_len: usize,
        ) -> i32;
    }
}
