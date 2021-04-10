use std::fmt;
use std::cmp;

#[derive(fmt::Debug, cmp::PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let expected = SomeOptions { foo: 32, bar: 99 };

        let options: SomeOptions = Default::default();
        assert_eq!(options, expected);
        println!("{:?}", options);

        let result = redraw().unwrap_or_default();
        assert_eq!(options, result);
        println!("{:?}", result)
    }
}