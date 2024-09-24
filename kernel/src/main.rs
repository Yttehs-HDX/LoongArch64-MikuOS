#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod uart;

#[no_mangle]
fn rust_main() -> ! {
    uart::uart_init();
    uprintln!("Hello, world!");
    loop {}
}