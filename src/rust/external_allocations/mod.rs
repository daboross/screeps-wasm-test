use std::mem;

pub fn alloc_bytes_uninitialized(length: usize) -> *mut u8 {
    let uninitialized_vec = Vec::<u8>::with_capacity(length);

    assert_eq!(uninitialized_vec.capacity(), length);

    let ptr = uninitialized_vec.as_slice().as_ptr() as *mut u8;

    mem::forget(uninitialized_vec);

    ptr
}

pub fn alloc_bytes_zeroed(length: usize) -> *mut u8 {
    let boxed_slice = vec![0u8; length].into_boxed_slice();

    assert_eq!(boxed_slice.len(), length);

    Box::into_raw(boxed_slice) as *mut u8
}

pub unsafe fn deallocate_exact(ptr: *mut u8, length: usize) {
    if length == 0 {
        return;
    }
    assert!(ptr as usize != 0);

    Vec::from_raw_parts(ptr, length, length);
}
