# Bank of Resources for `Rust Fundamentals`

### General Information

### Rust Memory layout

### Macros
Rust has excellent support for macros. Macros enable you to write code that writes other code, which is known as metaprogramming.

Sources:
- Overview: https://www.youtube.com/watch?v=dZiWkbnaQe8
- Syntax Fragments (and more): https://danielkeep.github.io/tlborm/book/mbe-macro-rules.html
- Explanation: https://blog.logrocket.com/macros-in-rust-a-tutorial-with-examples/#whatarerustmacros

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