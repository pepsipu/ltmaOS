use x86_64::structures::idt::PageFaultErrorCode;
use x86_64::structures::idt::InterruptStackFrame;
use x86_64::registers::control::Cr2;
use crate::{println, print};

pub extern "x86-interrupt" fn page_fault_handler(stack_frame: &mut InterruptStackFrame, error_code: PageFaultErrorCode) {
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    panic!("meanie don't touch memory that doesn't belong to you!!! >:(");
}