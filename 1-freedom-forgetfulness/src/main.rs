use std::alloc::{self, Layout};

/// Example of Forgetting to free memory:
fn main() {
    let layout = Layout::new::<i32>();
    let _ptr = unsafe { alloc::alloc(layout) as *mut i32 };
    // Missing deallocation, memory leak occurs
}
