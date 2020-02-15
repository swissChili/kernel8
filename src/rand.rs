use crate::mmio;

const CTRL: u32 = mmio::BASE + 0x00104000;
const STATUS: u32 = mmio::BASE + 0x00104004;
const DATA: u32 = mmio::BASE + 0x00104008;
const INT_MASK: u32 = mmio::BASE + 0x00104010;

pub struct Rand {}

impl Rand {
    pub fn new() -> Self {
        // TODO: init  hardware random-number generator
        mmio::write(STATUS, 0x40000);
        mmio::write(INT_MASK, mmio::read(INT_MASK) | 1);
        mmio::write(CTRL, mmio::read(CTRL) | 1);
        
        while (mmio::read(STATUS) >> 24) == 0 {
            //asm!("nop");
        }
        return Self {}
    }

    pub fn range(&self, min: u32, max: u32) -> u32 {
        return mmio::read(DATA) % (max - min) + min;
    }
}
