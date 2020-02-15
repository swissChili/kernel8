use crate::*;

#[repr(u8)]
#[allow(dead_code)]
pub enum Channel {
    Power = 0,
    Fb = 1,
    Vuart = 2,
    Vchiq = 3,
    Leds = 4,
    Bins = 5,
    Touch = 6,
    Count = 7,
    Prop = 8,
}

//#[repr(align(16))]
pub type MailBox = [u32; 36];

const VIDEOCORE: u32 = mmio::BASE + 0x0000B880;
const READ: u32 = VIDEOCORE + 0x0;
#[allow(dead_code)]
const POLL: u32 = VIDEOCORE + 0x10;
#[allow(dead_code)]
const SENDER: u32 = VIDEOCORE + 0x14;
const STATUS: u32 = VIDEOCORE + 0x18;
#[allow(dead_code)]
const CONFIG: u32 = VIDEOCORE + 0x1C;
const WRITE: u32 = VIDEOCORE + 0x20;
const RESPONSE: u32 = 0x80000000;
const FULL: u32 = 0x80000000;
const EMPTY: u32 = 0x40000000;
pub const REQUEST: u32 = 0;

#[macro_export]
macro_rules! mailbox {
    ( $( $x:expr ),+ ) => {{
        let mut b: $crate::mailbox::MailBox = [0; 36];
        let mut i = 0;
        $(
            b[i] = $x as u32;
            i += 1;
        )+

        b
    }}
}

pub trait Message {
    type Ret;
    fn to_array(&self) -> MailBox;
    fn get_result(&self, data: MailBox) -> Result<Self::Ret, ()>;

    fn call(&self, ch: Channel) -> Result<Self::Ret, ()> {
        let b = &self.to_array();
        let address: u32 = (b as *const MailBox as u32) & !0xF
                        | ((ch as u8) & 0xF) as u32;

        while mmio::read(STATUS) & FULL != 0 {
            nop!();
        }

        mmio::write(WRITE, address);

        loop {
            while mmio::read(STATUS) & EMPTY != 0 {
                nop!();
            }

            if address == mmio::read(READ) {
                if b[1] == RESPONSE {
                    serial::write("Got Response: ");
                    return self.get_result(*b);
                }
                else { return Err(()); }
            }
        }
    }
}

pub struct Request {
    pub tag: Tag,
    pub arg: [u32; 2],
}

impl Request {
    pub fn new(tag: Tag, arg: [u32; 2]) -> Self {
        return Self {
            tag: tag, arg: arg,
        }
    }
}

impl Message for Request {
    type Ret = u64;
    fn to_array(&self) -> MailBox {
        mailbox![
            8 * 4, REQUEST,
            self.tag,
            8, 8,
            self.arg[0], self.arg[1],
            Tag::Last
        ]
    }

    fn get_result(&self, data: MailBox) -> Result<Self::Ret, ()> {
        Ok((data[5] as u64) << 32 & data[6] as u64)
    }
}

pub fn get_serial() -> Result<u64, ()> {
    Request::new(Tag::GetSerial, [3424, 23423]).call(Channel::Prop)
}

#[repr(u32)]
#[allow(dead_code)]
#[derive(Clone, Copy)]
pub enum Tag {
    GetSerial = 0x10004,
    SetPower = 0x28001,
    SetClkRate = 0x38002,
    Last = 0,
}
