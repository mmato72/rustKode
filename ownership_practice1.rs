fn main() {
    let val1 = create_string();
    println!("String inside function: {}", val1);
}

fn create_string() -> String {
    let some_string = String::from("hello");
    some_string
}