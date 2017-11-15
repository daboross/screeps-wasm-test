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
pub unsafe extern "C" fn debug_room_position(ptr: *mut u8, len: usize) {
    let input = decoding::read_ptr(ptr, len);

    functionality::debug_room_position(input).expect("TODO: output useful error here");
}

#[no_mangle]
pub unsafe extern "C" fn debug_room_position_consuming(ptr: *mut u8, len: usize) {
    let input = decoding::consume_ptr(ptr, len);

    functionality::debug_room_position(&input).expect("TODO: output useful error here");
}

mod imports {
    extern "C" {
        pub fn print_str(ptr: *mut u8, len: usize);
    }
}
