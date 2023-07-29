use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    data: i32,
    // Circular reference using Rc and RefCell
    next: Option<Rc<RefCell<Node>>>,
}

/// Example of: Circular references in reference-counted (Rc<T>) or shared (Arc<T>) smart pointers:
fn main() {
    let node1 = Rc::new(RefCell::new(Node { data: 1, next: None }));
    let node2 = Rc::new(RefCell::new(Node { data: 2, next: None }));

    // Creating the circular reference
    node1.borrow_mut().next = Some(node2.clone());
    node2.borrow_mut().next = Some(node1.clone());

    // At this point, the reference counts of node1 and node2 will never reach 0,
    // and the nodes won't be deallocated, causing a memory leak.
}
