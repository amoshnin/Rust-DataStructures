// Information
// Rust has excellent support for macros. Macros enable you to write code that writes other code, which is known as metaprogramming.

// Sources:
// - Overview: https://www.youtube.com/watch?v=dZiWkbnaQe8
// - Syntax Fragments (and more): https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
// - Explanation: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#whatarerustmacros

fn main() {

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sum_macro_test() {
        let result = sum_macro!(2, 4, 6);
        assert_eq!(result, 12);
    }

    #[test]
    fn sum_vector_macro() {
        let result = vec_macro!(1,2,3,4,5);
        assert_eq!(result, vec![1,2,3,4,5]);
    }
}

#[macro_export]
macro_rules! vec_macro {
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        }
    }
}

#[macro_export]
macro_rules! sum_macro {
    ( $($x: expr), +) => {
        {
            let mut acc = 0;
            $( acc += $x; )+
            acc
        }
    }
}

