fn main() {
    let a = 5;
    let a = a + 1; // Shadowing the first a
    println!("The value of a is: {}", a);

    {
        let a = a * 2; // Shadowing within a new scope
        println!("The value of a in the inner scope is: {}", a);
    }
    
    println!("The value of a is: {}", a);
}