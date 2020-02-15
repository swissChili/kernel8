#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, asm)]

extern crate numtoa;

mod mmio;
mod io;
mod rand;
mod delay;
mod fb;
mod mailbox;
use io::serial;
use core::panic::PanicInfo;
use numtoa::NumToA;
use mailbox::Message;

#[macro_export]
macro_rules! nop {
    () => { unsafe { asm!("nop"); } }
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial::writeln("Kernel Panic:");
    loop {
    }
}

pub fn get_el() -> u64 {
    let mut val: u64;
    unsafe { asm!("mrs $0, CurrentEL" : "=r"(val)); }
    return val >> 2;
}

#[no_mangle]
pub extern "C" fn main() -> ! {
    serial::writeln("Kernel8");
    let rand = rand::Rand::new();
    let mut buffer: [u8; 4] = [0; 4];

    delay::wait_msec(300);

    serial::writeln("Serial Number is:");
    if let Ok(serial_number) = mailbox::get_serial() {
        serial::write_hex(serial_number);
    } else {
        serial::writeln("BRUH");
    }

    let req = fb::FrameBufferRequest::new(1024, 768);
    if let Ok(buff) = req.call(mailbox::Channel::Prop) {
        serial::writeln("Got Framebuffer!");
        serial::write_hex(buff.buffer as *const u32 as u64);
    } else {
        serial::writeln("Could not get framebuffer!");
        panic!("Could not get framebuffer");
    }

    serial::write("Exception level: ");
    serial::writeln(get_el().numtoa_str(10, &mut buffer));
    serial::writeln("Press any key to get a random number");

    loop {
        let c = serial::getc();
        serial::write("'");
        serial::writec(c);
        serial::write("'");
        serial::writeln(rand.range(0, 999).numtoa_str(10, &mut buffer));
        serial::write_hex(delay::get_sys_time());
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() {
    loop {}
}
