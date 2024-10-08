// From https://gitee.com/zeng_yu_chao/laos
// 3A5000 寄存器使用手册 163 页

const UART0_ADDR: u64 = 0x1fe001e0u64;

#[allow(unused)]
pub const RHR: *mut u8 = UART0_ADDR as *mut u8;
pub const THR: *mut u8 = UART0_ADDR as *mut u8;
pub const DLL: *mut u8 = UART0_ADDR as *mut u8;
pub const IER: *mut u8 = (UART0_ADDR + 1) as *mut u8;
pub const DLM: *mut u8 = (UART0_ADDR + 1) as *mut u8;
#[allow(unused)]
pub const FCR: *mut u8 = (UART0_ADDR + 1) as *mut u8;
#[allow(unused)]
pub const ISR: *mut u8 = (UART0_ADDR + 2) as *mut u8;
pub const LCR: *mut u8 = (UART0_ADDR + 3) as *mut u8;
#[allow(unused)]
pub const MCR: *mut u8 = (UART0_ADDR + 4) as *mut u8;
pub const LSR: *mut u8 = (UART0_ADDR + 5) as *mut u8;
#[allow(unused)]
pub const MSR: *mut u8 = (UART0_ADDR + 6) as *mut u8;
#[allow(unused)]
pub const SPR: *mut u8 = (UART0_ADDR + 7) as *mut u8;

#[allow(unused)]
pub const LSR_RX_READY: u8 = 1u8 << 0;
pub const LSR_TX_IDEL: u8 = 1u8 << 5;