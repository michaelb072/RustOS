#![feature(lang_items)]
#![feature(unique)]
#![feature(ptr_internals)]
#![feature(const_fn)]
#![no_std]

#[macro_use]
mod vga_buffer;

extern crate volatile;
extern crate spin;

#[no_mangle]
pub extern fn rust_main() {
    vga_buffer::clear_screen();
    println!("Hello world");
    println!("{}", { println!("Inner"); "outer" });
    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
