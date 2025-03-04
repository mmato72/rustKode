fn main() {
    greet("Alice");
    greet("Bob");
}

fn greet(name: &str) {
    println!("Hello, {}!", name);
}