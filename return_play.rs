fn main() {
    let result = add(10, 4);
    println!("The sum is: {}", result);
}

fn add(a: i32, b: i32) -> i32 {
    return a + b
}