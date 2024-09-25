use core::fmt::{Arguments, Write};
use crate::sbi;

// region Console begin
struct Console;

impl Write for Console {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        sbi::uart_print_str(s);
        Ok(())
    }
}
// region Console end

pub fn print(args: Arguments) {
    Console.write_fmt(args).unwrap();
}

#[macro_export]
macro_rules! print {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::console::print(format_args!($fmt $(, $($arg)+)?));
    };
}

#[macro_export]
macro_rules! println {
    ($fmt: literal $(, $($arg: tt)+)?) => {
        $crate::print!($fmt $(, $($arg)+)?);
        $crate::print!("\n");
    };
}