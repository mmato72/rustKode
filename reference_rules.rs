fn main() {
    let mut s = String::from("hello");

    {
        let r1 = &s; // Immutable reference
        let r2 = &s; // Another immutable reference
        println!("r1: {}, r2: {}", r1, r2); // Both refs are valid 
    } // Immutable references go out of scope here

    let r3 = &mut s; // Mutable reference
    r3.push_str(", world");
    println!("r3: {}", r3); // Valid mut ref
}