use alloc::string::String;
use crate::programs::shell::*;
use crate::println;
use alloc::vec::Vec;
use core::intrinsics::transmute;

pub fn init(args: Vec<&str>) {
    let mut bytecode: Vec<u8> = Vec::new();
    let mut content: Vec<&str> = args[1].split("-").collect();
    for num in content {
        bytecode.push(num.parse::<u8>().unwrap());
    }
    /*for file in FS.lock().iter() {
        if file.name == args[1] {
            content = file.contents.clone();
        }
    }
    println!("loaded {}", args[1]);*/

    let function = unsafe {
        transmute::<*const _, fn()>(&bytecode as *const _)
    };
    function();
}