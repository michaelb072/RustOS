#![feature(ptr_internals)]
#![feature(const_fn)]
#![no_std]

#[macro_use]
mod vga_buffer;

use core::panic::PanicInfo;

extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello world");
    println!("{}", { println!("Inner"); "outer" });
    loop{}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
