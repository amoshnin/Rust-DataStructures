#[derive(Debug, Clone, Copy)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

#[derive(Debug, Clone)] // since not all fields of the struct implement the `Copy` trait -> the String (which is heap allocated)
pub struct Person { // therefore the struct neither can implement the Copy trait
    name: String, // heap allocated (therefore doesn't implement the `Copy` trait)
    age: i32
}

// String cannot implement the `Copy` trait, because as any other variable sized container (eg: Vec)
// - it contains a pointer to some variable amount of heap memory.
// The only correct way to copy String is to allocate new block of heap memory and copy all contents inside there. - That is what the `Clone` implemenation does

// Everything is stored on the stack in some fashion. Even when you heap-allocate something, the pointer itself is stored on the stack.

// Everything in Rust can be moved. If this wasn't true, then the language would be very hard to use! Your data is moved from one stack frame to another.
// The only difference between a "copy" and a "move" is a copy allows both the source and destination values to be used, while a move only allows the destination value to be used.

fn main() {
    let p1 = Point::new(3, 6);
    let p2 = p1;

    println!("{:?}, {:?}", p1, p2)
}