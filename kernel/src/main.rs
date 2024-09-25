#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod sbi;
mod console;

#[no_mangle]
fn rust_main() -> ! {
    sbi::uart_init();
    println!("Hello, world!");
    sbi::uart_shutdown();
}