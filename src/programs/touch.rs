use alloc::string::String;
use crate::programs::shell::*;
use crate::println;
use alloc::vec::Vec;

pub fn init(file_args: Vec<&str>) {
    FS.lock().push(File {
        name: String::from(file_args[1]),
        contents: Vec::new()
    })
}