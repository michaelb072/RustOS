#![feature(lang_items)]
#![no_std]

#[no_mangle]
pub extern fn rust_main() {
    let hello = b"Michael <3 Lele";
    let color_byte = 0x1f; // white foreground blue background

    // let mut hello_colord = [color_byte; 24];
    let mut hello_colord = [color_byte; 40];

    for (i, char_byte) in hello.into_iter().enumerate() {
        hello_colord[i*2] = *char_byte;
    }

    let buffer_ptr = (0xb8000 + 1988) as *mut _;
    unsafe { *buffer_ptr = hello_colord };

    loop{}
}

#[lang = "eh_personality"] #[no_mangle] pub extern fn eh_personality() {}
#[lang = "panic_fmt"] #[no_mangle] pub extern fn panic_fmt() -> ! {loop{}}
