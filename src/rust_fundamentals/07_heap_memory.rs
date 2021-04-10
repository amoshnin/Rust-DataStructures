#[derive(Debug)]
struct LinkedList<T> {
    data: T,
    next: Option<Box<LinkedList<T>>>,
}

impl<T: std::ops::AddAssign> LinkedList<T> {
    pub fn add_up(&mut self, n: T) {
        self.data += n;
    }
}

fn main() {
    let mut ll = LinkedList::<i32> {
        data: 3,
        next: Some(Box::new(LinkedList {
            data: 2,
            next: None,
        })),
    };

    if let Some(ref mut value) = ll.next {
        let k = &mut **value;
        k.add_up(10);
    }

    println!("{:#?}", ll);

    // Vector - Dynamically sized array (stored on the heap) (variable sized container)
    // note: (cannot implement the Copy trait) - because: https://users.rust-lang.org/t/cant-derive-copy-because-of-string/18665
    // It has:
     // - Length (how many elements it has)
     // - Capacity (how much memory have I already allocated for holding this, can expand)
     //   (when expanding they allocate twice as much memory as they already have, copy the data across and deallocate the old memory)
     // - Pointer to the actual data
    let mut v: Vec<String> = Vec::new();
    // let mut v: Vec<String> = Vec::with_capacity(100);
    v.push("Hello".to_string());
    v.push("Goodbye".to_string());
    println!("v.len = {}, capacity = {}", v.len(), v.capacity());

    for i in 0..=2 {
        v.push("Hello".to_string());
    }
    println!("v.len = {}, capacity = {}", v.len(), v.capacity());
}
