/// An of a failure to remove items from a Vec collection:
fn main() {
    let mut data = vec![1, 2, 3, 4, 5];

    // Some code that operates on `data` and removes elements from it
    for num in &data {
        if *num % 2 == 0 {
            // Fails to remove elements correctly
            // This will lead to a memory leak, as the removed elements will not be deallocated.
            let index = data.iter().position(|x| *x == *num);
            if let Some(i) = index {
                data.remove(i);
            }
        }
    }

    // The elements with even values should have been removed, but they are still in the Vec.
    println!("{:?}", data);
}
