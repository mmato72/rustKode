fn main() {
    // Using copy
    let x = 5; // i32 is Copy
    let y = x; // x is copied to y
    println!("x: {}, y: {}", x, y); // Both x and y are valid

    // Using Clone
    let s1 = String::from("Hello");
    let s2 = s1.clone(); // Explicit cloning
    println!("s1: {}, s2: {}", s1, s2);
}