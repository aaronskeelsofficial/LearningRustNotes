//    Box<> is used primarily in three scenarios:
//    1. You need to create an object passing in a generic with dynamic sizing due to different trait implemented options
//    2. Recursive types of any kind (storing a type within itself)
//    3. Using large data structures (stack can't fit that much data)
//    Note to self: Smart pointers such as Box, Rc, and Arc are the ONLY WAY written into the language to intentionally assign singular object values to the heap.
//    Vec and other growable collections also allow numerous object values to be on the heap, but ultimately use the above under the hood.

//    1
trait Shape {
}
struct Circle {
    radius: f64,
}
impl Shape for Circle {
}
struct Rectangle {
    width: f64,
    height: f64,
}
impl Shape for Rectangle {
}
fn main() {
    let shapes: Vec<Box<dyn Shape>> = vec![
        Box::new(Circle { radius: 2.0 }),
        Box::new(Rectangle { width: 3.0, height: 4.0 }),
    ];
}
//    This works only with smart pointers (box, rc, arc) because they themselves have a known size, one of their variables being a pointer to wherever the inner data is stored.
//    Do note: The Box is only necessary here specifically because Vec itself requires a trait bound of a type with known size. This is not a universal rule.

//    2
struct Node {
    value: i32,
    next: Option<Box<Node>>, // Using Box to store the next node
}
fn main() {
    let node1 = Box::new(Node {
        value: 1,
        next: None,
    });
    let node2 = Box::new(Node {
        value: 2,
        next: Some(node1),
    });
}
//    This works only with smart pointers (box, rc, arc) because they themselves have a known size, one of their variables being a pointer to wherever the inner data is stored.
//    Do note: The Box is only necessary here specifically because Vec itself requires a trait bound of a type with known size. This is not a universal rule.

//    3
struct LargeData {
    data: [u8; 10_000], // Large array
}
fn main() {
    let large_data = Box::new(LargeData {
        data: [0; 10_000],
    });
}
