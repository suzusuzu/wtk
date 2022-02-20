#![no_main]
#![no_std]

#[link(wasm_import_module = "host")]
extern {
  fn hello(x: i32);
}

fn hello_(x: i32) {
    unsafe {
        hello(x);
    }
}


#[no_mangle]
pub extern fn main() {
    hello_(1);
}


use core::panic::PanicInfo;
#[panic_handler]
fn panic(_panic: &PanicInfo<'_>) -> !{ loop {} }