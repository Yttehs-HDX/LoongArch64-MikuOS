#![no_std]
#![no_main]

#![feature(linkage)]

pub use wrapper::*;

mod lang_items;
mod syscall;
#[macro_use]
pub mod console;
pub mod wrapper;

#[linkage = "weak"]
#[no_mangle]
fn main() -> i32 {
    panic!("Default main should not be called");
}

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    unreachable!()
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