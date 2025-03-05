fn main() {
    let result = subtract(10, 4);
    println!("The difference is: {}", result);
}

fn subtract(a: i32, b: i32) -> i32 {
    return a - b
}