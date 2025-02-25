macro_rules! greet {
    () => {
        println!("Hello, Rustaceans!");
    };
}

fn main() {
    greet!();
}