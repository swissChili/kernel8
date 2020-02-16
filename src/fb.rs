use crate::*;
use core::intrinsics::transmute;

pub struct FrameBuffer {
    pub buffer: *mut u32,
    pub width: u32,
    pub height: u32,
    pub real_width: u32,
    pub real_height: u32,
    pub pitch: u32,
    pub is_rgb: u32,
}

impl FrameBuffer {
    fn set_pixel(&self, x: u32, y: u32, color: u32) {
        unsafe {
            *((self.buffer as u64 + (y * self.width + x) as u64)
                as *mut u32) = color;
        }
    }

    pub fn render(&self) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_pixel(x, y, 0xff_00_00_00);
            }
        }
    }
}

pub struct FrameBufferRequest {
    width: u32,
    height: u32,
}

impl FrameBufferRequest {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width, height
        }
    }
}

impl mailbox::Message for
FrameBufferRequest {
    type Ret = FrameBuffer;

    fn to_array(&self) -> mailbox::MailBox {
        mailbox![
            35 * 4,
            mailbox::REQUEST,

            0x48003, // physical wh
            8,
            8,
            self.width,
            self.height,

            0x48004, // virtual wh
            8,
            8,
            self.width,
            self.height,

            0x48009,
            8,
            8,
            0,
            0,

            0x48005, // depth
            4,
            4,
            32,

            0x48006, // pixel order (rgb)
            4,
            4,
            1,

            0x40001, // get frame buffer
            8,
            8,
            4096, // ptr
            0,
            
            0x40008,
            4,
            4,
            0,

            mailbox::Tag::Last
        ]
    }

    fn get_result(&self, data: mailbox::MailBox)
    -> Result<Self::Ret, ()> {
        if data[20] == 32 && data[28] != 0 {
            return Ok(FrameBuffer {
                height: self.height,
                width: self.width,
                real_width: data[5],
                real_height: data[6],
                pitch: data[33],
                is_rgb: data[24],
                buffer: unsafe {
                    transmute((data[28] & 0x3FFFFFFF) as u64)
                },
            });
        }
        Err(())
    }
}
