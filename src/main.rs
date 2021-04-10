// Information
// Rust has excellent support for macros. Macros enable you to write code that writes other code, which is known as metaprogramming.

// Sources:
// - Overview: https://www.youtube.com/watch?v=dZiWkbnaQe8
// - Syntax Fragments (and more): https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
// - Explanation: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#whatarerustmacros

fn main() {
    let z = printer!(432, 43, 432, 4, 34, 3, 43, 4, 344, 4);
    println!("{:?}", z);
}

#[macro_export]
macro_rules! printer { // declaration (name of the macro)
    ( $($x: expr), *) => {
        {
            let mut temp_vec = Vec::new();
        $(
        temp_vec.push($x);
        )*
        temp_vec
        }
    }
}
