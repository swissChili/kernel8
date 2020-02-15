pub mod serial {
    use crate::mmio::{
        self,
        transmit_fifo_full,
        receive_fifo_empty,
        UART_DR
    };
    use numtoa::NumToA;

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
}
