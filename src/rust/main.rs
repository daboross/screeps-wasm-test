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
    unsafe {
        3 + callback(5)
    }
}

#[no_mangle]
pub extern "C" fn alloc_bytes(total: usize) -> *mut u8 {
    let slice = vec![0; total].into_boxed_slice();
    assert_eq!(slice.len(), total);

    // cast from *mut [u8] to *mut u8 to get rid of size in pointer
    Box::into_raw(slice)
}

#[no_mangle]
pub extern "C" fn sum(input: *mut u8, len: usize) -> i32 {
    let input = unsafe {
        std::slice::from_raw_parts(input, length)
    };
    input.iter().map(|&x| x as i32).sum();
}

#[no_mangle]
pub extern "C" fn dealloc_bytes(input: *mut u8, len: usize) {
    unsafe {
        Box::from_raw(std::slice::from_raw_parts_mut(input, length) as *mut _);
    }
}

#[no_mangle]
pub extern "C" fn fib(times: i32) -> i32 {
    _fib(times)
}

extern "C" {
    pub fn callback(input: i32) -> i32;
}
