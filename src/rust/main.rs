extern crate quick_protobuf;

mod protobuf;

fn main() {}

#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> *mut u8 {
    let slice = vec![0; len].into_boxed_slice();
    assert_eq!(slice.len(), len);

    // cast from *mut [u8] to *mut u8 to get rid of size in pointer
    Box::into_raw(slice) as *mut u8
}

#[no_mangle]
pub extern "C" fn dealloc_bytes(input: *mut u8, len: usize) {
    unsafe {
        Box::from_raw(std::slice::from_raw_parts_mut(input, len) as *mut _);
    }
}

#[no_mangle]
pub extern "C" fn debug_room_position(input: *mut u8, len: usize) {
    utils::debug_str(&format!(
        "debug_room_position: received: (ptr {} -> len {})",
        input as usize,
        len
    ));

    let input = unsafe { std::slice::from_raw_parts(input, len) };

    utils::debug_slice(input);

    let mut reader = quick_protobuf::BytesReader::from_bytes(input);

    let position: protobuf::pos::RoomPosition = reader
        .read_message(input, protobuf::pos::RoomPosition::from_reader)
        .unwrap();

    utils::debug_str(&format!("debug_room_position: {:#?}", position));
}

mod utils {
    use external;
    pub fn debug_slice(slice: &[u8]) {
        let ptr = slice.as_ptr() as *mut u8;
        let len = slice.len();
        unsafe {
            external::print_bytes(ptr, len);
        }
    }
    pub fn debug_str(string: &str) {
        let ptr = string.as_bytes().as_ptr() as *mut u8;
        let len = string.as_bytes().len();
        unsafe {
            external::print_str(ptr, len);
        }
    }
}

mod external {
    extern "C" {
        pub fn print_bytes(ptr: *mut u8, len: usize);

        pub fn print_str(ptr: *mut u8, len: usize);
    }
}
