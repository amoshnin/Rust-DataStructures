use x_data::StylableMacro;
fn main() {}

// --> Builder Pattern for a Type Custom Derive Procedural Macro
#[cfg(test)]
mod builder_tests {
    use super::*;

    #[test]
    fn builder_derive_macro_test() {}
}

// --> Reference Custom Derive Procedural Macro Example
pub trait Stylable {
    fn restyle() -> String;
}

#[derive(StylableMacro)]
pub struct Food {
    pub name: String,
}

#[test]
fn builder_derive_macro_test() {}

#[cfg(test)]
mod examples_tests {
    use super::*;

    #[test]
    fn stylable_derive_macro_test() {
        assert_eq!(Food::restyle(), "Food");
    }
}

// mod sorting_algorithms;
// use sorting_algorithms::merge_sort;
//
// fn main() {
//     // let mut v = vec![22, 6, 11, 8, 12, 2, 4];
//
// }
