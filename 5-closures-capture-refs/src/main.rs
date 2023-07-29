/// Example of memory leak from closures capturing references
fn main() {
    let data = vec![1, 2, 3];

    // Closure capturing a reference to `data`
    let closure = || {
        // Do something with `data`
        println!("{:?}", data);
    };

    // The closure is stored or passed around but never used,
    // and it keeps the reference to `data` alive longer than needed, leading to a memory leak.
}
