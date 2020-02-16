pub mod serial {
    extern crate spin;
    use crate::mmio::{
        self,
        transmit_fifo_full,
        receive_fifo_empty,
        UART_DR
    };
    use spin::Mutex;
    use numtoa::NumToA;
    use core::fmt;

    pub fn writec(c: u8) {
        while transmit_fifo_full() {}
        mmio::write(UART_DR, c as u32);
    }

    pub fn getc() -> u8 {
        while receive_fifo_empty() {}
        mmio::read(UART_DR) as u8
    }

    pub fn write(s: &str) {
        for c in s.chars() {
            writec(c as u8);
        }
    }

    pub fn writeln(s: &str) {
        write(s);
        writec(b'\n');
    }

    pub fn write_hex(s: u64) {
        let mut buf: [u8; 24] = [0; 24];
        write("0x");
        writeln(s.numtoa_str(16, &mut buf));
    }

    pub struct Writer;

    impl fmt::Write for Writer {
        fn write_str(&mut self, s: &str) -> fmt::Result {
            write(s);
            Ok(())
        }
    }

    pub static mut SERIAL_WRITER: Mutex<Writer> = Mutex::new(Writer{});

    #[macro_export]
    macro_rules! print {
        ($($arg:tt)*) => (
            $crate::io::serial::_print(format_args!($($arg)*))
        );
    }

    #[macro_export]
    macro_rules! println {
        () => ({$crate::print!("\n")});
        ($($arg:tt)*) => ({
            $crate::print!("{}\n", format_args!($($arg)*))
        });
    }

    pub fn _print(args: fmt::Arguments) {
        use core::fmt::Write;
        unsafe {
            SERIAL_WRITER.lock().write_fmt(args).unwrap();
        }
    }

    #[test_case]
    fn test_println() {
        let mut writer = Writer{};
        println!("Testing println!() macro");
    }
}
