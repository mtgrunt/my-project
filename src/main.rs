use my_project::{error_handling, hashmaps, strings, vectors, Guess};

fn main() {
    println!("=== Chapter 8: Common Collections ===");

    println!("\n--- 8.1 Vectors ---");
    vectors::demo();

    println!("\n--- 8.2 Strings ---");
    strings::demo();

    println!("\n--- 8.3 Hash Maps ---");
    hashmaps::demo();

    println!("\n=== Chapter 9: Error Handling ===");

    println!("\n--- 9.1 / 9.2  panic! and Result ---");
    error_handling::demo();

    println!("\n--- 9.3 Guess: custom validation type ---");
    let g = Guess::new(42);
    println!("[9.3] Guess::new(42).value() = {}", g.value());
    println!("[9.3] Guess::new(0) or Guess::new(101) would panic at construction.");
}
