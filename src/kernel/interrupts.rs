use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use lazy_static::lazy_static;
use pic8259_simple::ChainedPics;
use spin;
use crate::{println, print};
use crate::kernel::gdt;
use x86_64::instructions::port::Port;
use pc_keyboard::{Keyboard, ScancodeSet1, DecodedKey, layouts};

pub mod handlers;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> = spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

lazy_static! {
        static ref KEYBOARD: spin::Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = spin::Mutex::new(Keyboard::new(layouts::Us104Key, ScancodeSet1));
}

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt[Interrupts::Timer.as_usize()].set_handler_fn(handlers::timer::timer_handler);
        idt[Interrupts::Keyboard.as_usize()].set_handler_fn(handlers::keyboard::keyboard_handler);
        idt.page_fault.set_handler_fn(handlers::page_fault::page_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(handlers::double_fault::double_fault).set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}


#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum Interrupts {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl Interrupts {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}