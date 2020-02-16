#![no_std]
#![no_main]
#![feature(core_intrinsics, lang_items, asm, custom_test_frameworks)]
#![feature(panic_info_message)]
#![test_runner(crate::test::runner)]
#![reexport_test_harness_main = "test_main"]

extern crate numtoa;

mod mmio;
#[macro_use]
mod io;
mod rand;
mod delay;
mod fb;
mod mailbox;
mod test;
use io::serial;
use core::fmt::Write;
use core::panic::PanicInfo;
use numtoa::NumToA;
use mailbox::Message;
use fb::Color;

#[macro_export]
macro_rules! nop {
    () => { unsafe { asm!("nop"); } }
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("Kernel Panic:\n\t{}", info.message().expect("Unknown"));
    if let Some(loc) = info.location() {
        println!("\tIn file {}:{}:{}", loc.file(), loc.line(), loc.column());
    }
    loop { }
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

    println!("Kernel8");
    let rand = rand::Rand::new();

    delay::wait_msec(300);

    /*println!("Serial Number is:");
    if let Ok(serial_number) = mailbox::get_serial() {
        println!("{}", serial_number);
    } else {
        println!("Could not get serial number");
        panic!();
    }*/

    let req = fb::FrameBufferRequest::new(1024, 768);
    let buff = req.call(mailbox::Channel::Prop).unwrap();

    println!("Exception level: {}", get_el());
    println!("Press any key to get a random number");

    loop {
        let c = serial::getc();
        //println!("'{}'", c);
        //println!("{}", rand.range(0, 100));
        //buff.char_at(0, 0, c)
        buff.set_pixel(rand.range(0, 100), rand.range(0, 100), Color::white());
    }
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern fn _Unwind_Resume() {
    loop {}
}

#[test_case]
fn test_get_el() {
    println!("Testing: get_el() == 3");

    assert!(get_el() == 2);
}
