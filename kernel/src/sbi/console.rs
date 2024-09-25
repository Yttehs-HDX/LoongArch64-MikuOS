use super::{LSR, LSR_RX_READY, LSR_TX_IDEL, RHR, THR};

pub fn uart_print_str(str: &str) {
    unsafe {
        for b in str.as_bytes() {
            while (LSR.read() & LSR_TX_IDEL) == 0 {};
            THR.write_volatile(*b);
        }
    }
}

pub fn uart_get_char() -> u8 {
    unsafe {
        while (LSR.read() & LSR_RX_READY) == 0 {};
        return RHR.read();
    }
}