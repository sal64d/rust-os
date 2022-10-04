use volatile::Volatile;
use core::fmt;

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ScreenColor(u8);

impl ScreenColor {
    pub fn new(fg: Color, bg: Color) -> ScreenColor{
        ScreenColor((bg as u8) << 4 | (fg as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
struct ScreenChar {
    ascii_char: u8,
    color_code: ScreenColor,
}

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT]
}

pub struct Writer {
    col_pos: usize,
    color_code: ScreenColor,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8){

        match byte {
            b'\n' => self.line_break(),
            byte => {
                if self.col_pos >= BUFFER_WIDTH {
                    self.line_break();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.col_pos;
                self.buffer.chars[row][col].write(ScreenChar {
                    ascii_char: byte,
                    color_code: self.color_code
                });

                self.col_pos += 1;
            }
        }
    }

    pub fn line_break(&mut self) {
        for j in 1..BUFFER_HEIGHT {
            for i in 0..BUFFER_WIDTH {
                self.buffer.chars[j-1][i].write(
                    self.buffer.chars[j][i].read()
                );
            }
        }

        self.line_clear(BUFFER_HEIGHT -1);
        self.col_pos = 0;
    }

    fn line_clear(&mut self, row: usize) {
        for i in 0..BUFFER_WIDTH {
            self.buffer.chars[row][i].write(ScreenChar {
                ascii_char: b' ',
                color_code: ScreenColor::new(Color::Black, Color::Black)
            });
        }
    }

    pub fn write_string(&mut self, s: &str){
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

use lazy_static::lazy_static;
use spin::Mutex;

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        col_pos: 0,
        color_code: ScreenColor::new(Color::Yellow, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) }
    });
}

// macro time
#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    WRITER.lock().write_fmt(args).unwrap();
}
