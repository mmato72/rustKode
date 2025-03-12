fn main() {
    let difference = calculate_difference(10, 3);
    println!("The difference is: {}", difference);
}

fn calculate_difference(a: i32, b: i32) -> i32 {
    {
        let result = a - b;
        result
    }
}