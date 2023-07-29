# Rust-Memory-Leaks
A compendium of examples of Rust memory leaks:

1. **Forgetting to free memory**: If you use raw pointers (`*mut T` or `*const T`) directly and forget to call the `dealloc` function, memory won't be deallocated properly, resulting in a memory leak.
    
2. **Circular references in reference-counted (`Rc<T>`) or shared (`Arc<T>`) smart pointers**: When two or more objects reference each other using reference-counted or shared smart pointers, they can create a cycle where their reference counts never reach zero, causing memory leaks.
    
3. **Unclosed resources**: Rust's memory safety extends beyond just heap memory. If you have unclosed resources like files, network connections, or shared memory, they can lead to memory leaks.
    
4. **Global variables**: Storing large data structures in global variables can prevent them from being deallocated until the program's termination, resulting in memory leaks.
    
5. **Closures capturing references**: Closures can capture references to variables from their surrounding environment. If these closures are stored or passed around, they can keep references alive longer than necessary, causing memory leaks.
    
6. **Unsafe code**: While unsafe code is necessary in some cases, using it incorrectly can lead to memory leaks. In unsafe blocks, Rust's safety guarantees are bypassed, and manual memory management becomes the programmer's responsibility.
    
7. **Failure to remove items from collections**: If you store items in a data structure like a `Vec` or `HashMap` and forget to remove them when they are no longer needed, the data structure will retain unnecessary memory.