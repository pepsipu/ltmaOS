use crate::programs::shell::FS;
use crate::println;
pub fn init() {
    for file in FS.lock().iter() {
        println!("{}", file.name);
    }
}