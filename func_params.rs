fn main() {
    let area = calculate_area(5, 10);
    println!("The area is: {}", area);
}

fn calculate_area(width: i32, height: i32) -> i32 {
    width * height
}