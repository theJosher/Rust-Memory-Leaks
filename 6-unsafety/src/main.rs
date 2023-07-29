/// Example of memory leak from unsafe code:
fn main() {
    let layout = std::alloc::Layout::from_size_align(8, 4).unwrap();
    let ptr = unsafe { std::alloc::alloc(layout) };
    // Some code using `ptr` unsafely
    // Missing deallocation, memory leak occurs in case of an early return or other failure.
    unsafe {
        std::alloc::dealloc(ptr, layout);
    }
}
