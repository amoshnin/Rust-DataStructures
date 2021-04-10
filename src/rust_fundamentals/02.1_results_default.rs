#[derive(Debug)]
pub struct SomeOptions {
    foo: i32,
    bar: i32,
}

impl std::default::Default for SomeOptions {
    fn default() -> Self {
        Self { foo: 32, bar: 99 }
    }
}

fn redraw() -> Result<SomeOptions, String> {
    Err("good morning".to_string())
}

fn main() {
    let options: SomeOptions = Default::default();
    println!("{:?}", options);

    let result = redraw().unwrap_or_default();
    println!("{:?}", result)
}
