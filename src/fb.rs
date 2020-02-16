use crate::*;
use io::*;
use core::intrinsics::transmute;

// Exported by font.o
extern "C" {
    #[no_mangle]
    pub static mut _binary_src_font_psf_start: u8;
}

#[repr(packed)]
#[derive(Copy, Clone, Debug)]
pub struct PSFHeader {
    pub magic: u32,
    pub version: u32,
    pub header_size: u32,
    pub flags: u32,
    pub num_glyph: u32,
    pub bytes_per_glyph: u32,
    pub height: u32,
    pub width: u32,
    pub glyphs: u8,
}

#[derive(Debug)]
pub struct ScreenFont {
    pub header: PSFHeader,
    pub start: *mut u8,
}

impl ScreenFont {
    pub fn new(from: *mut u8) -> Self {
        unsafe {
            let header = *(from as *mut PSFHeader);
            Self {
                header,
                start: (from as usize + header.header_size as usize)
                    as *mut u8,
            }
        }
    }

    pub fn glyph(&self, glyph: u8) -> *mut u8 {
        (self.start as usize +
            (glyph as usize) *
            self.header.bytes_per_glyph as usize)
            as *mut u8
    }

    pub fn debug_glyph(&self, c: u8) {
        let mut glyph = self.glyph(c);
        for i in 0 .. self.header.height {
            for j in 0 .. self.header.width {
                print!("{}",
                    if unsafe {*glyph} & (0b1000_0000 >> j) > 0 {"#"} else {" "}
                );
            }
            println!();
            glyph = (glyph as usize + self.header.width as usize / 8) as *mut u8;
        }
    }
}

#[derive(Debug)]
pub struct FrameBuffer {
    pub buffer: *mut u32,
    pub width: u32,
    pub height: u32,
    pub real_width: u32,
    pub real_height: u32,
    pub pitch: u32,
    pub is_rgb: u32,
    pub font: ScreenFont,
}

fn cap(num: u32, low: u32, high: u32) -> u32 {
    if num < low {
        low
    } else if num > high {
        high
    } else {
        num
    }
}

#[repr(packed)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

#[allow(unused)]
impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self {
            r, g, b, a,
        }
    }

    pub fn red() -> Self {
        return Self::new(255, 0, 0, 255);
    }

    pub fn white() -> Self {
        Self::new(0, 0, 0, 255)
    }
    
    pub fn black() -> Self {
        Self::new(0, 0, 0, 0)
    }
}

impl Into<u32> for Color {
    fn into(self) -> u32 {
        (self.r as u32) << 24
        | (self.g as u32) << 16
        | (self.b as u32) << 8
        | (self.a as u32)
    }
}

impl FrameBuffer {
    pub fn set_pixel(&self, x: u32, y: u32, color: Color) {
        unsafe {
            *((self.buffer as u64 + (y * self.width + x) as u64)
                as *mut u32) = color.into();
        }
    }

    pub fn char_at(&self, x: u32, y: u32, c: u8) {
        let mut glyph = self.font.glyph(c);
        let header = self.font.header;

        for i in 0 .. header.height {
            for j in 0 .. header.width {
                self.set_pixel(x + j, y + i,
                    if unsafe {*glyph} & (0b1000_0000 >> j) > 0 {
                        Color::white()
                    } else {
                        Color::black()
                    }
                );
            }
            glyph = (glyph as usize + header.width as usize / 8) as *mut u8;
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

impl mailbox::Message for FrameBufferRequest {
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
            0, // x offset
            0, // y offset

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
                font: unsafe {
                    ScreenFont::new(
                        &mut _binary_src_font_psf_start
                        as *mut u8
                    )
                },
            });
        }
        Err(())
    }
}


#[test_case]
fn test_get_frame_buffer() {
    println!("Testing get FrameBuffer");
    let req = FrameBufferRequest::new(1080, 720);
    if let Ok(buff) = req.call(mailbox::Channel::Prop) {
        println!("Got FrameBuffer, rendering");
        buff.render();
    } else {
        println!("Could not get FrameBuffer");
        panic!("Could not get FrameBuffer");
    }
}
