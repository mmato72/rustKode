
fn main() {
    let pair = (10, 20);

    match pair {
        (_, y) => println!("The second value is: {}", y),
    }
}
