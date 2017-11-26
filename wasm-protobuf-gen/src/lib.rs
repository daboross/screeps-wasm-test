#[macro_use]
extern crate proc_macro_hack;

#[allow(unused_imports)]
#[macro_use]
extern crate wasm_protobuf_gen_impl;
#[doc(hidden)]
pub use wasm_protobuf_gen_impl::*;

proc_macro_item_decl! {
    __js_fn_inner! => __js_fn_impl
}

#[macro_export]
macro_rules! js_fn {
    ($(fn $name:ident ($($args:tt)*) => $implementation:expr;)*) => {
        __js_fn_inner! {
            $(
                fn $name ($($args)*) {
                    $implementation
                }
            )*
        }
    };
}
