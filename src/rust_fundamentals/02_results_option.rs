pub enum Res<T, E> {
    Thing(T),
    Error(E),
}

fn divide(a: i32, b: i32) -> Res<i32, String> {
    if (b != 0) {
        let res = a / b;
        return Res::Thing(res)
    } else {
        return Res::Error("You cannot divide by zero".to_string())
    }
}

fn main() {
    let res = divide(2, 0);
    let res2 = divide(10, 5);

    if let Res::Thing(v) = res2 {
        println!("{}", v)
    }

    handle_error(&res);
    handle_error(&res2);
}


fn handle_error<T, E>(response: &Res<T, E>)
where T: std::fmt::Display,
      E: std::fmt::Display,
{
    match response {
        Res::Thing(result) => println!("Result is {}", result),
        Res::Error(error) => println!("Error is {}", error),
    }
}