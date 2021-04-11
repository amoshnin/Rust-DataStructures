fn main() {
    let a: i32 = 10; // (v1)

    if 1 == 1 {
        println!("We can see the a : {}", a); // 10
        let a: f32 = 23.3; // <- Shadowing (v2) // The upper value (v2) is still maintained
        // The inner scope sees the new variable (can be with different type)
        println("Inner scope {}", a);  // 23.3 (v2)
    }

    println("Outer scope {}", a);
}