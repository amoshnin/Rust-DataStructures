# Bank of Resources for `Rust Fundamentals`

### General Information

### Rust Memory layout

### Macros
Rust has excellent support for macros. Macros enable you to write code that writes other code, which is known as metaprogramming.

Sources:
- Overview: https://www.youtube.com/watch?v=dZiWkbnaQe8
- Syntax Fragments (and more): https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
- Explanation: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#whatarerustmacros
- Workshop: https://github.com/dtolnay/proc-macro-workshop

```rust
macro_rules! vector {
    ($elem: expr; $n: expr) => { }
}
```
 Declaration of the macro (declares the name of the macro)
 Patterns for macros (arguments to the macro (not like to the function))
 Instead of having a variable followed by a type, we have a `Syntax Pattern`
 The type is kind of a `Syntax Type` (eg: identifier, expression, block...)


Declarative Macros can be invoked in following ways (different delimiters):
```rust
vector!();
vector![];
vector!{};
```

Note: When returning anything from the declarative macro, we add an additional scope `{}` around everything.
This also allows us to store variables within that scope.
Below two examples are shown with test implementations:

```rust
// Without a return value (we are just mutating the `x` argument)
#[macro_export]
macro_rules! sum_up {
    ($x: ident; $($num: expr), *) => {
        $(
            for i in $num.iter() {
                $x += i;
            }
        )*
    }
}

// With a return value (we are creating a new vector, manipulating and returning it)
#[macro_export]
macro_rules! vec_macro {
    ( $($x: expr), *) => {
        { // <- this scope is added
            let mut temp_vec = Vec::new();
            $( temp_vec.push($x); )*
            temp_vec
        } // <- this scope is added
    }
}
```
Here we have the tests for these macros: 
```rust
#[test]
fn sum_macro_test() {
     let mut x = 1;
    sum_up![x; [2, 34, 5]];
    assert_eq!(x, 42);
}

#[test]
fn vector_macro_test() {
    let result = vec_macro!(1, 2, 3, 4, 5);
    assert_eq!(result, vec![1, 2, 3, 4, 5]);
}
```