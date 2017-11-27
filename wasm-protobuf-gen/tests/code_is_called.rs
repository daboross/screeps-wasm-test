#[macro_use]
extern crate wasm_protobuf_gen;

use std::sync::atomic::{AtomicBool, Ordering};

thread_local! {
    static HI1_RUN: AtomicBool = AtomicBool::new(false);
    static HI2_RUN: AtomicBool = AtomicBool::new(false);
}

mod moda {
    use std::sync::atomic::Ordering;
    use HI2_RUN;

    pub fn hi(_x: &[u8], _y: &mut [u8]) {
        HI2_RUN.with(|hi2_run| {
            hi2_run.store(true, Ordering::SeqCst);
        })
    }
}

js_fn! {
    fn hi(_: &[u8]) {
        HI1_RUN.with(|hi1_run| {
            hi1_run.store(true, Ordering::SeqCst);
        });
    }
    fn bye(x: &(([((u8))])), y: &mut [u8]) => moda::hi;
}

#[test]
fn printit() {
    HI1_RUN.with(|hi1_run| {
        assert_eq!(hi1_run.load(Ordering::SeqCst), false);
    });

    __js_fn_hi(0 as *const u8, 0usize);

    HI2_RUN.with(|hi2_run| {
        assert_eq!(hi2_run.load(Ordering::SeqCst), false);
    });

    __js_fn_bye(0 as *const u8, 0usize, 0 as *mut u8, 0usize);

    HI1_RUN.with(|hi1_run| {
        assert_eq!(hi1_run.load(Ordering::SeqCst), true);
    });

    HI2_RUN.with(|hi2_run| {
        assert_eq!(hi2_run.load(Ordering::SeqCst), true);
    });
}
