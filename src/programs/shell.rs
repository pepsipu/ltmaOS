use lazy_static::lazy_static;
use crate::kernel::vga::WRITER;
use crate::print;
use pc_keyboard::KeyCode;
use alloc::vec::Vec;
use alloc::string::String;
use spin::Mutex;

pub struct File {
    pub name: String,
    pub contents: Vec<u8>,
}

lazy_static! {
    pub static ref INPUT_BUFFER: spin::Mutex<Vec<char>> = spin::Mutex::new(Vec::new());
}

lazy_static! {
    pub static ref FS: spin::Mutex<Vec<File>> = spin::Mutex::new(Vec::new());
}

pub fn key_handle(character: char) {
    let mut buffer = INPUT_BUFFER.lock();
    if character == '\n' {
        print!("\n");
        let input: String = buffer.iter().collect();
        let args: Vec<&str> = input.split(" ").collect();
        match args[0] {
            "ltmaos-editor" => crate::programs::ltmaos_editor::init(args),
            "pwd" => crate::programs::pwd::init(),
            "touch" => crate::programs::touch::init(args),
            "exec" => crate::programs::exec::init(args),
            "write" => crate::programs::write::init(args),
            "ls" => crate::programs::ls::init(),
            _ => print!("{} not found >:(", args[0])
        }
        buffer.clear();
        print!("\n$ ");
    } else {
        buffer.push(character);
        print!("{}", character);
    }
}

pub fn special_key_handle(code: KeyCode) {
    if code == KeyCode::Backspace {}
}