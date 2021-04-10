#[derive(Debug)]
pub struct Person { // Structure of data
    name: String,
    age: i32,
    children: i32,
    favorite_color: Color,
}

impl Person {
    pub fn print(self) -> String {
        return format!("name = {}, age = {}, children = {}", self.name, self.age, self.children);
    }
}

#[derive(Debug)]
pub enum Color { // We can have different types of each of the associated data of the variants
    Red(String), // Rust allocates the amount of memory for the enum - of the biggest of the associated data
    Green(i32),
    Blue,
}

fn main() {
    let p = Person {
        name: String::from("Artem"),
        age: 2,
        children: 2,
        favorite_color: Color::Red("Yeah".to_string()),
    };

    let c = Color::Red( String::from("food") );
    match c {
        Color::Red(string) => println!("Red, {}", string),
        Color::Blue => println!("Blue"),
        Color::Green(num) => println!("Green {}", num),
    };

    println!("Hello, world, from \n {:#?}", p);
}
