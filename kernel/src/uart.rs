use core::fmt::{Arguments, Write};

// 3A5000 寄存器使用手册163页
const UART0_ADDR: u64 = 0x1fe001e0u64;
#[allow(dead_code)]
const RHR: *mut u8 = (UART0_ADDR + 0) as *mut u8;
const THR: *mut u8 = (UART0_ADDR + 0) as *mut u8;
const DLL: *mut u8 = (UART0_ADDR + 0) as *mut u8;
const IER: *mut u8 = (UART0_ADDR + 1) as *mut u8;
const DLM: *mut u8 = (UART0_ADDR + 1) as *mut u8;
#[allow(dead_code)]
const FCR: *mut u8 = (UART0_ADDR + 1) as *mut u8;
#[allow(dead_code)]
const ISR: *mut u8 = (UART0_ADDR + 2) as *mut u8;
const LCR: *mut u8 = (UART0_ADDR + 3) as *mut u8;
#[allow(dead_code)]
const MCR: *mut u8 = (UART0_ADDR + 4) as *mut u8;
const LSR: *mut u8 = (UART0_ADDR + 5) as *mut u8;
#[allow(dead_code)]
const MSR: *mut u8 = (UART0_ADDR + 6) as *mut u8;
#[allow(dead_code)]
const SPR: *mut u8 = (UART0_ADDR + 7) as *mut u8;
#[allow(dead_code)]
const LSR_RX_READY: u8 = 1u8 << 0;
const LSR_TX_IDEL: u8 = 1u8 << 5;

pub fn uart_init() {
    unsafe {
        IER.write(0x00);
        let mut lcr = LCR.read();
        LCR.write(lcr | (1 << 7));
        DLL.write(0x03);
        DLM.write(0x00);
        lcr = 0;
        LCR.write(lcr | (3 << 0));
    }
}

pub fn uart_print_str(str: &str) {
    unsafe {
        for b in str.as_bytes() {
            while (LSR.read() & LSR_TX_IDEL) == 0 {};
            THR.write_volatile(*b);
        }
    }
}

#[allow(unused)]
pub fn uart_get_char() -> u8 {
    unsafe {
        while (LSR.read() & LSR_RX_READY) == 0 {};
        return RHR.read();
    }
}

struct Console;
impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        uart_print_str(s);
        Ok(())
    }
}

pub fn _print(args: Arguments) {
    Console.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! uprint {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::_print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! uprintln {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::uart::_print(format_args!(concat!($fmt, "\n") $(, $($arg)+)?));
    };
}