use super::{DLL, DLM, IER, LCR};

pub fn uart_init() {
    unsafe {
        IER.write(0x00);
        let mut lcr = LCR.read();
        LCR.write(lcr | (1 << 7));
        DLL.write(0x03);
        DLM.write(0x00);
        lcr = 0;
        LCR.write(lcr | 3);
    }
}

pub fn uart_shutdown() -> ! {
    loop {}
}