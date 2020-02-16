#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, asm, custom_test_frameworks)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate numtoa;

mod mmio;
mod io;
mod rand;
mod delay;
mod fb;
mod mailbox;
mod test;
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
    #[cfg(test)]
    {
        test_main();
        loop { }
    }

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
    let buff = req.call(mailbox::Channel::Prop).unwrap();

    serial::write("Exception level: ");
    serial::writeln(get_el().numtoa_str(10, &mut buffer));
    serial::writeln("Press any key to get a random number");

    loop {
        let c = serial::getc();
        serial::write("'");
        serial::writec(c);
        serial::write("'");
        serial::writeln(rand.range(0, 999).numtoa_str(10, &mut buffer));
        //serial::write_hex(delay::get_sys_time());

        buff.render()
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() {
    loop {}
}
