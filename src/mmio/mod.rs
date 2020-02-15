use core::intrinsics::volatile_load;
use core::intrinsics::volatile_store;

pub const UART_DR: u32 = 0x3F201000;
pub const UART_FR: u32 = 0x3F201018;
pub const BASE: u32 = 0x3F000000;

pub fn write(reg: u32, val: u32) {
    unsafe {
        volatile_store(reg as *mut u32, val)
    }
}

pub fn read(reg: u32) -> u32 {
    unsafe {
        volatile_load(reg as *const u32)
    }
}

pub fn transmit_fifo_full() -> bool {
    read(UART_FR) & (1 << 5) > 0
}

pub fn receive_fifo_empty() -> bool {
    read(UART_FR) & (1 << 4) > 0
}
