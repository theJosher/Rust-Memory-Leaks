use std::fs::File;
use std::io::Write;

/// Example of memory leak from unclosed resources:
fn main() {
    let mut file = File::create("example.txt").expect("Failed to create file");
    file.write_all(b"Hello, world!").expect("Failed to write to file");
    // The file is not explicitly closed here, leading to a resource leak.
    // In a real program, use the `file` variable and call `.close()` or let it go out of scope.
}
