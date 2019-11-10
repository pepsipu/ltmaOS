use x86_64::structures::idt::InterruptStackFrame;
use crate::{println, print};

extern "x86-interrupt" fn syscall_handler(stack_frame: &mut InterruptStackFrame) {
    println!("syscall: stop giving me your stack frame >:( {:#?}", stack_frame);
}