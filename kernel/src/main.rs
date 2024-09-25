#![no_std]
#![no_main]

#![feature(alloc_error_handler)]

use core::arch::global_asm;

global_asm!(include_str!("entry.S"));

mod lang_items;
mod sbi;
#[macro_use]
mod console;
mod util;

#[no_mangle]
fn rust_main() -> ! {
    clear_bss();
    sbi::uart_init();
    util::logger_init();
    println!("Hello, world!");
    sbi::uart_shutdown();
}

fn clear_bss() {
    unsafe {
        let len = ebss as usize - sbss as usize;
        core::slice::from_raw_parts_mut(sbss as *mut u8, len).fill(0);
    }
}

extern "C" {
    fn sbss();
    fn ebss();
}