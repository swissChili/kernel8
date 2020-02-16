use crate::*;

const TIMER_BASE: u32 = 0x7E003000;
const TIMER_LOW: u32 = TIMER_BASE + 0x4;
const TIMER_HIGH: u32 = TIMER_BASE + 0x8;

pub fn wait_cycles(mut n: u32) {
    while n > 0 {
        unsafe { asm!("nop"); }
        n -= 1;
    }
}

pub fn get_sys_time() -> u64 {
    let mut h = mmio::read(TIMER_HIGH);
    let mut l = mmio::read(TIMER_LOW);

    if h != mmio::read(TIMER_HIGH) {
        l = mmio::read(TIMER_LOW);
        h = mmio::read(TIMER_HIGH);
    }

    ((h as u64) << 32) | (l as u64)
}

pub fn wait_msec(n: u32) {
    let mut freq: u32;
    let mut count: u32;
    let mut real: u32;

    unsafe {
        asm!("mrs $0, cntfrq_el0" : "=r"(freq));
        asm!("mrs $0, cntpct_el0" : "=r"(count));
    }

    count += ((freq / 1000) * n) / 1000;
    println!("{}", count);
    
    // do...while loops aren't a thing for some reason
    while {
        unsafe {
            asm!("mrs $0, cntpct_el0" : "=r"(real));
        }
        println!("{}", real);
        wait_cycles(10000000);

        real < count
    }{}
}
