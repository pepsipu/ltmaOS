use x86_64::structures::idt::InterruptStackFrame;
use crate::{print};
use crate::kernel::interrupts::{PICS, Interrupts};
use x86_64::instructions::port::Port;
use crate::kernel::interrupts::{KEYBOARD};
use spin;
use lazy_static::lazy_static;
use pc_keyboard::{DecodedKey, KeyCode};

pub enum CharType {
    Regular,
    Special
}

lazy_static! {
    pub static ref CURRENT_PROCESS: spin::Mutex<(fn(char), fn(KeyCode))> = spin::Mutex::new((crate::programs::shell::key_handle, crate::programs::shell::special_key_handle));
}

pub extern "x86-interrupt" fn keyboard_handler(stack_frame: &mut InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(0x60);
    let code: u8 = unsafe {port.read()};
    if let Ok(Some(key_event)) = keyboard.add_byte(code) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            let curr = CURRENT_PROCESS.lock();
            let character_f = curr.0;
            let special = curr.1;
            drop(curr);
            match key {
                DecodedKey::Unicode(character) => character_f(character),
                DecodedKey::RawKey(key) => special(key)
            }
        }
    }

    unsafe {
        PICS.lock().notify_end_of_interrupt(Interrupts::Keyboard.as_u8());
    }
}