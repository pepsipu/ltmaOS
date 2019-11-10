use alloc::vec::Vec;
use crate::kernel::interrupts::handlers::keyboard::CURRENT_PROCESS;
use crate::programs::shell;
use pc_keyboard::KeyCode;
use lazy_static::lazy_static;
use spin;
use alloc::string::String;
use crate::print;

lazy_static! {
    pub static ref FILE: spin::Mutex<(String,)> = spin::Mutex::new((String::from(""),));
}

lazy_static! {
    pub static ref RUN: spin::Mutex<(bool,)> = spin::Mutex::new((false, ));
}

lazy_static! {
    pub static ref BUFFER: spin::Mutex<Vec<u8>> = spin::Mutex::new(Vec::new());
}
pub fn init(args: Vec<&str>) {
    let mut curr = CURRENT_PROCESS.lock();
    let mut run = RUN.lock();
    run.0 = true;
    drop(run);
    curr.0 = key_handle;
    curr.1 = special_key_handle;
    drop(curr);
    FILE.lock().0 = String::from(args[1]);
    let mut stop = false;
    while !stop {
        x86_64::instructions::hlt();
        let running = RUN.lock();
        if running.0 {
            stop = true;
        }
        drop(running);
    }
}

pub fn key_handle(character: char) {
    print!("{}", character);
    if character == '\n' {
        let fs = shell::FS.lock();
        for file in fs.iter() {
            print!("{}", file.name);
        }
        let mut curr = CURRENT_PROCESS.lock();
        curr.0 = shell::key_handle;
        curr.1 = shell::special_key_handle;
    } else {
        BUFFER.lock().push(character as u8);
    }
}

pub fn special_key_handle(code: KeyCode) {

}