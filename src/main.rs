use x_data::{Builder, StylableMacro};
fn main() {}

// --> Builder Pattern for a Type Custom Derive Procedural Macro
#[derive(Builder)]
struct _Command {
    executable: String,
    args: Vec<String>,
    env: Vec<String>,
    current_dir: String,
}

pub fn feature_1() {}

#[cfg(test)]
mod builder_macro_tests {
    use super::*;

    #[test]
    fn builder_derive() {}
}

// --> Reference Custom Derive Procedural Macro Example
trait Stylable {
    fn restyle() -> String;
}

#[derive(StylableMacro)]
struct Food;

#[cfg(test)]
mod examples_macro_tests {
    use super::*;

    #[test]
    fn stylable_derive() {
        assert_eq!(Food::restyle(), "Food");
    }
}

#[allow(dead_code)]
mod restyle {
    pub fn jj() {}
}

// mod sorting_algorithms;
// use sorting_algorithms::merge_sort;
//
// fn main() {
//     // let mut v = vec![22, 6, 11, 8, 12, 2, 4];
//
// }
