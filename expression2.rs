fn main() {
    let number = 2;
    let result = match number {
        1 => "one",
        2 => "two",
        3 => "three",
        _ => "other",
    };
    println!("The result is: {}", result);
}