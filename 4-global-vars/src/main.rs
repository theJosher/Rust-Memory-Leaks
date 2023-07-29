use std::sync::Mutex;

static mut GLOBAL_DATA: Option<Mutex<Vec<i32>>> = None;

/// Example of memory leak from global variables:
fn main() {
    unsafe {
        GLOBAL_DATA = Some(Mutex::new(vec![1, 2, 3]));
        // More code...
        // The global data will persist until the program terminates, causing a memory leak.
    }
}
