use volatile::Volatile;
use lazy_static::lazy_static;
use spin::Mutex;
use core::fmt;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::kernel::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    use x86_64::instructions::interrupts;
    interrupts::without_interrupts(|| {
        WRITER.lock().write_fmt(args).unwrap();
    });

}

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column: 0,
        vga: unsafe {&mut *(0xb8000 as *mut Buffer)}
    });
}


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum ColorEnum {
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
pub struct Color(pub u8);
impl Color {
    pub(crate) fn new(foreground: ColorEnum, background: ColorEnum) -> Color {
        Color((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    pub char: u8,
    pub color: Color
}

#[repr(transparent)]
pub struct Buffer {
    pub chars: [[Volatile<Char>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    pub column: usize,
    pub vga: &'static mut Buffer
}

impl Writer {
    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.vga.chars[row][col].read();
                self.vga.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = Char {
            char: b' ',
            color: Color::new(ColorEnum::Black, ColorEnum::Black),
        };
        for col in 0..BUFFER_WIDTH {
            self.vga.chars[row][col].write(blank);
        }
    }

    pub fn write(&mut self, string: &str) {
        for byte in string.bytes()  {
            if byte == b'\n' {
                self.new_line();
            } else {
                if self.column >= BUFFER_WIDTH {
                    self.new_line();
                }
                self.vga.chars[BUFFER_HEIGHT - 1][self.column].write(Char {
                    char: byte,
                    color: Color::new(ColorEnum::Cyan, ColorEnum::Black)
                });
                self.column += 1;
            }
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, string: &str) -> fmt::Result {
        self.write(string);
        Ok(())
    }
}