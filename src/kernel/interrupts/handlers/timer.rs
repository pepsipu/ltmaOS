use x86_64::structures::idt::InterruptStackFrame;
use crate::kernel::interrupts::{Interrupts, PICS};

pub extern "x86-interrupt" fn timer_handler(stack_frame: &mut InterruptStackFrame) {
    unsafe {
        PICS.lock().notify_end_of_interrupt(Interrupts::Timer.as_u8());
    }
}