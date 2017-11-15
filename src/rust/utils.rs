use imports;

pub fn print<T: AsRef<str>>(thing: T) {
    let bytes = thing.as_ref().as_bytes();
    let ptr = bytes.as_ptr() as *mut u8;
    let len = bytes.len();
    unsafe {
        imports::print_str(ptr, len);
    }
}
