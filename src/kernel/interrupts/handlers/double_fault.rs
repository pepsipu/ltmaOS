use x86_64::structures::idt::InterruptStackFrame;



pub extern "x86-interrupt" fn double_fault(stack_frame: &mut InterruptStackFrame, err_code: u64) {
    panic!("double fault >:( err code: {} here's the stack frame: {:#?}", err_code, stack_frame);
}