fn main() {
    let pair = (10, 20);

    match pair {
        (x, _) => println!("The first value is: {}", x),
    }
}