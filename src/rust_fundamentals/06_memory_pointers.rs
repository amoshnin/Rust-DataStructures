#[derive(Debug)]
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }

    fn greet(&self) -> String {
        return format!("HI my name is {}", self.name);
    }

    fn rename_consuming(self) -> Self {
        Self { name: "Matt".to_string(), ..self }
    }

    fn age_up(&mut self, number: i32) {
        self.age += number;
    }

    fn drop(self) {}
}

fn main() {
    let p = Person::new("".to_string(), 24);
    println!("greet: {}", p.greet()); // (&self)

    let mut z = p.rename_consuming(); // (self)
    println!("greet: {}", z.greet()); // (&self)

    let age = get_age(&z); // immutable borrow to (&self)
    // z.age_up(32); // ERROR: `cannot borrow `z` as mutable because it is also borrowed as immutable`
    // WHY: Our access variables is controlled and restricted the borrows checker machnism.
    // to  `Age's` lifetime is a pointer to smth of `z`

    z.greet(); // But we do can use the immutable reference (&self)
    println!("the age is: {}", age);

    //////////////////////////////// //////////////////////////////// //////////////////////////////// ////////////////////////////////

    // NOW we can use `z` mutably again. Because Age is no longer needed and its lifetime of the immutable reference above have finished
    // This can be thought as scopes. We longer need the immutable reference, so we can use the mutable one.
    z.age_up(32); // mutable borrow to (&mut self) - 0.1
    // 0.1 dealocated (no longer needed)
    // so we can have another mutable reference
    z.age_up(32); // mutable borrow to (&mut self)
    z.drop();

    // z.age_up(32); dropped already
}

fn get_age(item: &Person) -> &i32 {
    return &item.age;
}