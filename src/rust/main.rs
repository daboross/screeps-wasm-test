extern crate quick_protobuf;

mod protobuf;

fn main() {}

fn _fib(input: i32) -> i32 {
    let mut last = 0;
    let mut current = 1;
    for _ in 0..input {
        let new = current + last;
        last = current;
        current = new;
    }
    current
}

#[no_mangle]
pub extern "C" fn entry() -> i32 {
    unsafe { 3 + external::callback(5) }
}

#[no_mangle]
pub extern "C" fn alloc_bytes(len: usize) -> *mut u8 {
    let slice = vec![0; len].into_boxed_slice();
    assert_eq!(slice.len(), len);

    utils::debug_str(&format!(
        "allocated at: {} -> {}",
        slice.as_ptr() as usize,
        (slice.as_ptr() as usize) + len
    ));

    // cast from *mut [u8] to *mut u8 to get rid of size in pointer
    Box::into_raw(slice) as *mut u8
}

#[no_mangle]
pub extern "C" fn sum(input: *mut u8, len: usize) -> i32 {
    let input = unsafe { std::slice::from_raw_parts(input, len) };
    input.iter().map(|&x| x as i32).sum()
}

#[no_mangle]
pub extern "C" fn retrieve_x(input: *mut u8, len: usize) -> i32 {
    utils::debug_str(&format!(
        "retrieve_x received: {} -> {}",
        input as usize,
        (input as usize) + len
    ));

    let input = unsafe { std::slice::from_raw_parts(input, len) };

    utils::debug_slice(input);

    let mut reader = quick_protobuf::BytesReader::from_bytes(input);

    let position: protobuf::pos::RoomPosition = reader
        .read_message(input, protobuf::pos::RoomPosition::from_reader)
        .unwrap();

    return position.x;
}

#[no_mangle]
pub extern "C" fn dealloc_bytes(input: *mut u8, len: usize) {
    unsafe {
        Box::from_raw(std::slice::from_raw_parts_mut(input, len) as *mut _);
    }
}

#[no_mangle]
pub extern "C" fn fib(times: i32) -> i32 {
    _fib(times)
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
        pub fn callback(input: i32) -> i32;

        pub fn print_bytes(ptr: *mut u8, len: usize);

        pub fn print_str(ptr: *mut u8, len: usize);
    }
}
