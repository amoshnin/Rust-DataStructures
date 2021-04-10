fn main() {
    let slice_var = "  hello  "; // slice pointing to that specific point of the binary
    let trimmed_slice_var = slice_var.trim(); // stack pointer to the same data in the binary as the `S` variable
                      // but with different length and start index (address)
                      // note: this makes it a very memory efficient process

    println!("{}", slice_var);
    println!("trimmed_slice_var == {}", trimmed_slice_var);


    let mut string_var = "world".to_string(); // The memory of the binary ("world") has been copied into the heap
                                 // Now `k` is pointing to that place on the heap

    let trimmed = string_var.trim();
    let trimmed = trimmed.to_string();

    string_var.push_str("dsa");
    println!("{}", trimmed);

    let fstr = "help me find home";
    let ffstr = string_find_f(fstr);
    println!("{}", ffstr);

    println!("{}", choose_str(1));
}

// String === Box<str>
// String is a `string slice` on the heap.
// Therefore the String is the same as Box<str>

fn string_find_f<'a>(text: &'a str) -> &'a str {
    for (index, item) in text.char_indices() {
        if item == 'f' {
            return &text[index..];
        }
    }
    return text;
}

fn choose_str(number: i32) -> &'static str {
    match number {
        1 => "Hello",
        2 => "World",
        _ => "Other thingy",
    }
}