use crate::kernel::vga::{WRITER, BUFFER_HEIGHT, BUFFER_WIDTH, Char, ColorEnum, Color};
use volatile::Volatile;
use alloc::string::String;
use alloc::vec::Vec;

struct TextEditor {
    x_position: u32,
    y_position: u32
}

pub fn init(file_args: Vec<&str>) {
    let mut writer = WRITER.lock();
    for i in 0..file_args[1].len() {
        writer.vga.chars[0][i].write(Char {
            char: file_args[1].as_bytes()[i] as u8,
            color: Color::new(ColorEnum::Black, ColorEnum::Cyan)
        });
    }
    for i in file_args[1].len()..BUFFER_WIDTH {
        writer.vga.chars[0][i].write(Char {
            char: b' ',
            color: Color::new(ColorEnum::Black, ColorEnum::Cyan)
        });
    }
    loop {
        x86_64::instructions::hlt();
    }
}

pub fn key_handle(character: char) {

}

pub fn special_key_handle(character: char) {

}